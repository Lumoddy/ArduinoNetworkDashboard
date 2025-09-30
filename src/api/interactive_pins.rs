use arduino_hal::hal::port;
use super::{InteractivePin, InteractivePinID, PinDigitalInteraction, PinDigitalInteractionChanges, PinGetModeError, PinGetPowerError, PinMode, PinModeInteraction, PinPowerChangeError, PinSetModeError, PinSetPowerError};

pub struct InteractivePins
{
    pub d2: InteractivePin<port::PD2>,
    pub d3: InteractivePin<port::PD3>,
    pub d4: InteractivePin<port::PD4>,
    pub d5: InteractivePin<port::PD5>,
    pub d6: InteractivePin<port::PD6>,
    pub d7: InteractivePin<port::PD7>,
    pub d8: InteractivePin<port::PB0>,
    pub d9: InteractivePin<port::PB1>,
    pub d10: InteractivePin<port::PB2>,
    pub d11: InteractivePin<port::PB3>,
    pub d12: InteractivePin<port::PB4>,
    pub d13: InteractivePin<port::PB5>,
    pub a0: InteractivePin<port::PC0>,
    pub a1: InteractivePin<port::PC1>,
    pub a2: InteractivePin<port::PC2>,
    pub a3: InteractivePin<port::PC3>,
    pub a4: InteractivePin<port::PC4>,
    pub a5: InteractivePin<port::PC5>,
}

pub enum DynamicInteractivePinBorrow<'a>
{
    D2(&'a mut InteractivePin<port::PD2>),
    D3(&'a mut InteractivePin<port::PD3>),
    D4(&'a mut InteractivePin<port::PD4>),
    D5(&'a mut InteractivePin<port::PD5>),
    D6(&'a mut InteractivePin<port::PD6>),
    D7(&'a mut InteractivePin<port::PD7>),
    D8(&'a mut InteractivePin<port::PB0>),
    D9(&'a mut InteractivePin<port::PB1>),
    D10(&'a mut InteractivePin<port::PB2>),
    D11(&'a mut InteractivePin<port::PB3>),
    D12(&'a mut InteractivePin<port::PB4>),
    D13(&'a mut InteractivePin<port::PB5>),
    A0(&'a mut InteractivePin<port::PC0>),
    A1(&'a mut InteractivePin<port::PC1>),
    A2(&'a mut InteractivePin<port::PC2>),
    A3(&'a mut InteractivePin<port::PC3>),
    A4(&'a mut InteractivePin<port::PC4>),
    A5(&'a mut InteractivePin<port::PC5>),
}

impl InteractivePins
{
    pub fn borrow<'a>(&'a mut self, pin: InteractivePinID)
        -> DynamicInteractivePinBorrow<'a>
    {
        match pin
        {
            InteractivePinID::D2 => DynamicInteractivePinBorrow::D2(&mut self.d2),
            InteractivePinID::D3 => DynamicInteractivePinBorrow::D3(&mut self.d3),
            InteractivePinID::D4 => DynamicInteractivePinBorrow::D4(&mut self.d4),
            InteractivePinID::D5 => DynamicInteractivePinBorrow::D5(&mut self.d5),
            InteractivePinID::D6 => DynamicInteractivePinBorrow::D6(&mut self.d6),
            InteractivePinID::D7 => DynamicInteractivePinBorrow::D7(&mut self.d7),
            InteractivePinID::D8 => DynamicInteractivePinBorrow::D8(&mut self.d8),
            InteractivePinID::D9 => DynamicInteractivePinBorrow::D9(&mut self.d9),
            InteractivePinID::D10 => DynamicInteractivePinBorrow::D10(&mut self.d10),
            InteractivePinID::D11 => DynamicInteractivePinBorrow::D11(&mut self.d11),
            InteractivePinID::D12 => DynamicInteractivePinBorrow::D12(&mut self.d12),
            InteractivePinID::D13 => DynamicInteractivePinBorrow::D13(&mut self.d13),
            InteractivePinID::A0 => DynamicInteractivePinBorrow::A0(&mut self.a0),
            InteractivePinID::A1 => DynamicInteractivePinBorrow::A1(&mut self.a1),
            InteractivePinID::A2 => DynamicInteractivePinBorrow::A2(&mut self.a2),
            InteractivePinID::A3 => DynamicInteractivePinBorrow::A3(&mut self.a3),
            InteractivePinID::A4 => DynamicInteractivePinBorrow::A4(&mut self.a4),
            InteractivePinID::A5 => DynamicInteractivePinBorrow::A5(&mut self.a5),
        }
    }
}

impl<'a> PinDigitalInteraction for DynamicInteractivePinBorrow<'a>
{
    fn pin_is_high(&self) -> Result<bool, PinGetPowerError>
    {
        match self
        {
            Self::D2(pin) => pin.pin_is_high(),
            Self::D3(pin) => pin.pin_is_high(),
            Self::D4(pin) => pin.pin_is_high(),
            Self::D5(pin) => pin.pin_is_high(),
            Self::D6(pin) => pin.pin_is_high(),
            Self::D7(pin) => pin.pin_is_high(),
            Self::D8(pin) => pin.pin_is_high(),
            Self::D9(pin) => pin.pin_is_high(),
            Self::D10(pin) => pin.pin_is_high(),
            Self::D11(pin) => pin.pin_is_high(),
            Self::D12(pin) => pin.pin_is_high(),
            Self::D13(pin) => pin.pin_is_high(),
            Self::A0(pin) => pin.pin_is_high(),
            Self::A1(pin) => pin.pin_is_high(),
            Self::A2(pin) => pin.pin_is_high(),
            Self::A3(pin) => pin.pin_is_high(),
            Self::A4(pin) => pin.pin_is_high(),
            Self::A5(pin) => pin.pin_is_high(),
        }
    }

    fn set_pin_is_high(&mut self, power: bool) -> Result<(), PinSetPowerError>
    {
        match self
        {
            Self::D2(pin) => pin.set_pin_is_high(power),
            Self::D3(pin) => pin.set_pin_is_high(power),
            Self::D4(pin) => pin.set_pin_is_high(power),
            Self::D5(pin) => pin.set_pin_is_high(power),
            Self::D6(pin) => pin.set_pin_is_high(power),
            Self::D7(pin) => pin.set_pin_is_high(power),
            Self::D8(pin) => pin.set_pin_is_high(power),
            Self::D9(pin) => pin.set_pin_is_high(power),
            Self::D10(pin) => pin.set_pin_is_high(power),
            Self::D11(pin) => pin.set_pin_is_high(power),
            Self::D12(pin) => pin.set_pin_is_high(power),
            Self::D13(pin) => pin.set_pin_is_high(power),
            Self::A0(pin) => pin.set_pin_is_high(power),
            Self::A1(pin) => pin.set_pin_is_high(power),
            Self::A2(pin) => pin.set_pin_is_high(power),
            Self::A3(pin) => pin.set_pin_is_high(power),
            Self::A4(pin) => pin.set_pin_is_high(power),
            Self::A5(pin) => pin.set_pin_is_high(power),
        }
    }
}

impl<'a> PinDigitalInteractionChanges for DynamicInteractivePinBorrow<'a>
{
    fn detect_pin_change(&mut self) -> Result<Option<bool>, PinPowerChangeError>
    {
        match self
        {
            Self::D2(pin) => pin.detect_pin_change(),
            Self::D3(pin) => pin.detect_pin_change(),
            Self::D4(pin) => pin.detect_pin_change(),
            Self::D5(pin) => pin.detect_pin_change(),
            Self::D6(pin) => pin.detect_pin_change(),
            Self::D7(pin) => pin.detect_pin_change(),
            Self::D8(pin) => pin.detect_pin_change(),
            Self::D9(pin) => pin.detect_pin_change(),
            Self::D10(pin) => pin.detect_pin_change(),
            Self::D11(pin) => pin.detect_pin_change(),
            Self::D12(pin) => pin.detect_pin_change(),
            Self::D13(pin) => pin.detect_pin_change(),
            Self::A0(pin) => pin.detect_pin_change(),
            Self::A1(pin) => pin.detect_pin_change(),
            Self::A2(pin) => pin.detect_pin_change(),
            Self::A3(pin) => pin.detect_pin_change(),
            Self::A4(pin) => pin.detect_pin_change(),
            Self::A5(pin) => pin.detect_pin_change(),
        }
    }
}

impl<'a> PinModeInteraction for DynamicInteractivePinBorrow<'a>
{
    fn pin_mode(&self) -> Result<PinMode, PinGetModeError>
    {
        match self
        {
            Self::D2(pin) => pin.pin_mode(),
            Self::D3(pin) => pin.pin_mode(),
            Self::D4(pin) => pin.pin_mode(),
            Self::D5(pin) => pin.pin_mode(),
            Self::D6(pin) => pin.pin_mode(),
            Self::D7(pin) => pin.pin_mode(),
            Self::D8(pin) => pin.pin_mode(),
            Self::D9(pin) => pin.pin_mode(),
            Self::D10(pin) => pin.pin_mode(),
            Self::D11(pin) => pin.pin_mode(),
            Self::D12(pin) => pin.pin_mode(),
            Self::D13(pin) => pin.pin_mode(),
            Self::A0(pin) => pin.pin_mode(),
            Self::A1(pin) => pin.pin_mode(),
            Self::A2(pin) => pin.pin_mode(),
            Self::A3(pin) => pin.pin_mode(),
            Self::A4(pin) => pin.pin_mode(),
            Self::A5(pin) => pin.pin_mode(),
        }
    }

    fn set_pin_mode(&mut self, mode: PinMode) -> Result<(), PinSetModeError>
    {
        match self
        {
            Self::D2(pin) => pin.set_pin_mode(mode),
            Self::D3(pin) => pin.set_pin_mode(mode),
            Self::D4(pin) => pin.set_pin_mode(mode),
            Self::D5(pin) => pin.set_pin_mode(mode),
            Self::D6(pin) => pin.set_pin_mode(mode),
            Self::D7(pin) => pin.set_pin_mode(mode),
            Self::D8(pin) => pin.set_pin_mode(mode),
            Self::D9(pin) => pin.set_pin_mode(mode),
            Self::D10(pin) => pin.set_pin_mode(mode),
            Self::D11(pin) => pin.set_pin_mode(mode),
            Self::D12(pin) => pin.set_pin_mode(mode),
            Self::D13(pin) => pin.set_pin_mode(mode),
            Self::A0(pin) => pin.set_pin_mode(mode),
            Self::A1(pin) => pin.set_pin_mode(mode),
            Self::A2(pin) => pin.set_pin_mode(mode),
            Self::A3(pin) => pin.set_pin_mode(mode),
            Self::A4(pin) => pin.set_pin_mode(mode),
            Self::A5(pin) => pin.set_pin_mode(mode),
        }
    }
}