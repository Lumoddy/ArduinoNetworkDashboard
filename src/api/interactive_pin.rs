use core::ptr;
use arduino_hal::hal::port;
use arduino_hal::port::{mode, PinOps};
use crate::json::{self, StringVisitor, Visitor};

#[derive(Clone, Copy)]
pub enum PinGetPowerError { }

#[derive(Clone, Copy)]
pub enum PinSetPowerError
{
    IsInput,
}

pub trait PinDigitalInteraction
{
    fn pin_is_high(&self) -> Result<bool, PinGetPowerError>;

    fn set_pin_is_high(&mut self, power: bool) -> Result<(), PinSetPowerError>;
}

#[derive(Clone, Copy)]
pub enum PinPowerChangeError
{
    IsOutput,
}

pub trait PinDigitalInteractionChanges
{
    fn detect_pin_change(&mut self) -> Result<Option<bool>, PinPowerChangeError>;
}

pub enum PinMode
{
    DigitalInput,
    DigitalOutput,
}

#[derive(Clone, Copy)]
pub enum PinGetModeError { }

#[derive(Clone, Copy)]
pub enum PinSetModeError { }

pub trait PinModeInteraction
{
    fn pin_mode(&self) -> Result<PinMode, PinGetModeError>;

    fn set_pin_mode(&mut self, mode: PinMode) -> Result<(), PinSetModeError>;
}

enum _PinState<PIN: PinOps>
{
    DigitalInput
    {
        pin: port::Pin<mode::Input<mode::PullUp>, PIN>,
        was_high: bool,
    },
    DigitalOutput
    {
        pin: port::Pin<mode::Output, PIN>,
    },
}

pub struct InteractivePin<PIN: PinOps>
{
    _state: _PinState<PIN>,
}

impl<PIN: PinOps> From<port::Pin<mode::Input<mode::PullUp>, PIN>> for InteractivePin<PIN>
{
    fn from(pin: port::Pin<mode::Input<mode::PullUp>, PIN>) -> Self
    {
        Self
        {
            _state: _PinState::DigitalInput { was_high: pin.is_high(), pin },
        }
    }
}

impl<PIN: PinOps> From<port::Pin<mode::Output, PIN>> for InteractivePin<PIN>
{
    fn from(pin: port::Pin<mode::Output, PIN>) -> Self
    {
        Self
        {
            _state: _PinState::DigitalOutput { pin },
        }
    }
}

impl<PIN: PinOps> PinDigitalInteraction for InteractivePin<PIN>
{
    fn pin_is_high(&self) -> Result<bool, PinGetPowerError>
    {
        match &self._state
        {
            _PinState::DigitalInput { pin, was_high: _ }
                => Ok(pin.is_high()),
            _PinState::DigitalOutput { pin }
                => Ok(pin.is_set_high()),
        }
    }

    fn set_pin_is_high(&mut self, power: bool) -> Result<(), PinSetPowerError>
    {
        match &mut self._state
        {
            _PinState::DigitalInput { pin: _, was_high: _ }
                => Err(PinSetPowerError::IsInput),
            _PinState::DigitalOutput { pin }
                => Ok(if power { pin.set_high() } else { pin.set_low() }),
        }
    }
}

impl<PIN: PinOps> PinDigitalInteractionChanges for InteractivePin<PIN>
{
    fn detect_pin_change(&mut self) -> Result<Option<bool>, PinPowerChangeError>
    {
        match &mut self._state
        {
            _PinState::DigitalInput { pin, was_high } =>
            {
                let is_high = pin.is_high();
                let has_changed = is_high != *was_high;
                *was_high = is_high;

                if has_changed { Ok(Some(is_high)) } else { Ok(None) }
            },
            _PinState::DigitalOutput { pin: _ }
                => Err(PinPowerChangeError::IsOutput),
        }
    }
}

impl<PIN: PinOps> PinModeInteraction for InteractivePin<PIN>
{
    fn pin_mode(&self) -> Result<PinMode, PinGetModeError>
    {
        match &self._state
        {
            _PinState::DigitalInput { pin: _, was_high: _ }
                => Ok(PinMode::DigitalInput),
            _PinState::DigitalOutput { pin: _ }
                => Ok(PinMode::DigitalOutput),
        }
    }

    fn set_pin_mode(&mut self, mode: PinMode) -> Result<(), PinSetModeError>
    {
        match mode
        {
            PinMode::DigitalInput => match unsafe { ptr::read(&self._state) }
            {
                _PinState::DigitalInput { pin: _, was_high: _ } => Ok(()),
                _PinState::DigitalOutput { pin } =>
                {
                    let pin = pin.into_pull_up_input();
                    self._state = _PinState::DigitalInput
                    {
                        was_high: pin.is_high(),
                        pin,
                    };

                    Ok(())
                },
            },
            PinMode::DigitalOutput => match unsafe { ptr::read(&self._state) }
            {
                _PinState::DigitalInput { pin, was_high: _ } =>
                {
                    self._state = _PinState::DigitalOutput
                    {
                        pin: pin.into_output(),
                    };

                    Ok(())
                },
                _PinState::DigitalOutput { pin: _ } => Ok(()),
            },
        }
    }
}

impl TryFrom<&str> for PinMode
{
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error>
    {
        match value
        {
            "input" | "digital-input" => Ok(PinMode::DigitalInput),
            "output" | "digital-output" => Ok(PinMode::DigitalOutput),
            _ => Err(()),
        }
    }
}

impl From<PinMode> for &'static str
{
    fn from(value: PinMode) -> Self
    {
        match value
        {
            PinMode::DigitalInput => "digital-input",
            PinMode::DigitalOutput => "digital-output",
        }
    }
}

impl json::IntoJSON for PinMode
{
    fn into_json<T: json::ValueTracer>(self, tracer: T) -> Result<T::Return, T::Error>
    {
        tracer.str(self.into())
    }
}

impl json::FromJSON for PinMode
{
    fn from_json<V: json::ValueVisitor>(visitor: V) -> Result<(Self, V::Return), V::Error>
    {
        match visitor.value()?
        {
            json::TypedValueVisitor::Object(visitor) => return Err(
                visitor.into_invalid_value_err("object", "pin mode")),
            json::TypedValueVisitor::Array(visitor) => return Err(
                visitor.into_invalid_value_err("array", "pin mode")),
            json::TypedValueVisitor::String(visitor) =>
            {
                let (pin, visitor) = visitor.collect_string::<16>()?;
                match pin.as_str().try_into()
                {
                    Ok(pin) => Ok((pin, visitor)),
                    Err(()) => return Err(
                        visitor.into_invalid_value_err("invalid pin mode", "pin mode")),
                }
            },
            json::TypedValueVisitor::Number(visitor) => return Err(
                visitor.into_invalid_value_err("number", "pin mode")),
            json::TypedValueVisitor::Boolean(visitor) => return Err(
                visitor.into_invalid_value_err("boolean", "pin mode")),
            json::TypedValueVisitor::Null(visitor) => return Err(
                visitor.into_invalid_value_err("null", "pin mode")),
        }
    }
}