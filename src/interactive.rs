// use core::ptr;

// use arduino_hal::port::{self, mode, PinOps};

// #[derive(Clone, Copy)]
// pub enum PinGetPowerError { }

// #[derive(Clone, Copy)]
// pub enum PinSetPowerError
// {
//     IsInput,
// }

// pub trait PinDigitalInteraction
// {
//     fn get_pin_is_high(&self) -> Result<bool, PinGetPowerError>;

//     fn set_pin_is_high(&mut self, power: bool) -> Result<(), PinSetPowerError>;
// }

// #[derive(Clone, Copy)]
// pub enum PinPowerChangeError
// {
//     IsOutput,
// }

// pub trait PinDigitalInteractionChanges
// {
//     fn detect_pin_change(&mut self) -> Result<Option<bool>, PinPowerChangeError>;
// }

// pub enum PinMode
// {
//     DigitalInput,
//     DigitalOutput,
// }

// #[derive(Clone, Copy)]
// pub enum PinGetModeError { }

// #[derive(Clone, Copy)]
// pub enum PinSetModeError { }

// pub trait PinModeInteraction
// {
//     fn get_pin_mode(&self) -> Result<PinMode, PinGetModeError>;

//     fn set_pin_mode(&mut self, mode: PinMode) -> Result<(), PinSetModeError>;
// }

// enum _PinState<PIN: PinOps>
// {
//     DigitalInput
//     {
//         pin: port::Pin<mode::Input<mode::PullUp>, PIN>,
//         was_high: bool,
//     },
//     DigitalOutput
//     {
//         pin: port::Pin<mode::Output, PIN>,
//     },
// }

// pub struct Pin<PIN: PinOps>
// {
//     _state: _PinState<PIN>,
// }

// impl<PIN: PinOps> From<port::Pin<mode::Input<mode::PullUp>, PIN>> for Pin<PIN>
// {
//     fn from(pin: port::Pin<mode::Input<mode::PullUp>, PIN>) -> Self
//     {
//         Self
//         {
//             _state: _PinState::DigitalInput { was_high: pin.is_high(), pin },
//         }
//     }
// }

// impl<PIN: PinOps> From<port::Pin<mode::Output, PIN>> for Pin<PIN>
// {
//     fn from(pin: port::Pin<mode::Output, PIN>) -> Self
//     {
//         Self
//         {
//             _state: _PinState::DigitalOutput { pin },
//         }
//     }
// }

// impl<PIN: PinOps> PinDigitalInteraction for Pin<PIN>
// {
//     fn get_pin_is_high(&self) -> Result<bool, PinGetPowerError>
//     {
//         match &self._state
//         {
//             _PinState::DigitalInput { pin, was_high: _ }
//                 => Ok(pin.is_high()),
//             _PinState::DigitalOutput { pin }
//                 => Ok(pin.is_set_high()),
//         }
//     }

//     fn set_pin_is_high(&mut self, power: bool) -> Result<(), PinSetPowerError>
//     {
//         match &mut self._state
//         {
//             _PinState::DigitalInput { pin: _, was_high: _ }
//                 => Err(PinSetPowerError::IsInput),
//             _PinState::DigitalOutput { pin }
//                 => Ok(if power { pin.set_high() } else { pin.set_low() }),
//         }
//     }
// }

// impl<PIN: PinOps> PinDigitalInteractionChanges for Pin<PIN>
// {
//     fn detect_pin_change(&mut self) -> Result<Option<bool>, PinPowerChangeError>
//     {
//         match &mut self._state
//         {
//             _PinState::DigitalInput { pin, was_high } =>
//             {
//                 let is_high = pin.is_high();
//                 let has_changed = is_high != *was_high;
//                 *was_high = is_high;

//                 if has_changed { Ok(Some(is_high)) } else { Ok(None) }
//             },
//             _PinState::DigitalOutput { pin: _ }
//                 => Err(PinPowerChangeError::IsOutput),
//         }
//     }
// }

// impl<PIN: PinOps> PinModeInteraction for Pin<PIN>
// {
//     fn get_pin_mode(&self) -> Result<PinMode, PinGetModeError>
//     {
//         match &self._state
//         {
//             _PinState::DigitalInput { pin: _, was_high: _ }
//                 => Ok(PinMode::DigitalInput),
//             _PinState::DigitalOutput { pin: _ }
//                 => Ok(PinMode::DigitalOutput),
//         }
//     }

//     fn set_pin_mode(&mut self, mode: PinMode) -> Result<(), PinSetModeError>
//     {
//         match mode
//         {
//             PinMode::DigitalInput => match unsafe { ptr::read(&self._state) }
//             {
//                 _PinState::DigitalInput { pin: _, was_high: _ } => Ok(()),
//                 _PinState::DigitalOutput { pin } =>
//                 {
//                     let pin = pin.into_pull_up_input();
//                     self._state = _PinState::DigitalInput
//                     {
//                         was_high: pin.is_high(),
//                         pin,
//                     };

//                     Ok(())
//                 },
//             },
//             PinMode::DigitalOutput => match unsafe { ptr::read(&self._state) }
//             {
//                 _PinState::DigitalInput { pin, was_high: _ } =>
//                 {
//                     self._state = _PinState::DigitalOutput
//                     {
//                         pin: pin.into_output(),
//                     };

//                     Ok(())
//                 },
//                 _PinState::DigitalOutput { pin: _ } => Ok(()),
//             },
//         }
//     }
// }