
// use arduino_hal::hal::port;
// use arduino_hal::port::{mode, Pin};
// use arduino_hal::Peripherals;
// use avr_device::interrupt;

// pub trait ChangeDetectablePin
// {
//     fn enable_change_detector(&mut self);

//     fn disable_change_detector(&mut self);

//     fn change_detector_is_enabled(&self) -> bool;

//     fn set_change_callback(&mut self, f: fn(is_high: bool));

//     fn remove_change_callback(&mut self);
// }

// macro_rules! pin_change_impl
// {
//     (
//         $(
//             impl for $pin:ty
//             {
//                 static $static:ident;
//                 $mask:ident << $mask_index:expr;
//                 pcie << $pcie_index:expr$(;)?
//             }
//         )+
//     ) =>
//     {
//         $(
//             static mut $static: Option<fn(is_high: bool)> = None;

//             impl<MODE: mode::Io> ChangeDetectablePin for Pin<MODE, $pin>
//             {
//                 fn enable_change_detector(&mut self)
//                 {
//                     let dp = unsafe { Peripherals::steal() };
//                     dp.EXINT.pcicr.modify(|r, w| w
//                         .pcie().bits(r.pcie().bits() | (1 << $pcie_index)));
//                     dp.EXINT.$mask.modify(|r, w| w
//                         .pcint().bits(r.pcint().bits() | (1 << $mask_index)));
//                 }

//                 fn disable_change_detector(&mut self)
//                 {
//                     let dp = unsafe { Peripherals::steal() };
//                     dp.EXINT.$mask.modify(|r, w|
//                     {
//                         if r.pcint().bits() == 1 << $mask_index
//                         {
//                             dp.EXINT.pcicr.modify(|r, w| w
//                                 .pcie().bits(r.pcie().bits() & !(1 << $pcie_index)));
//                         }

//                         w.pcint().bits(r.pcint().bits() & !(1 << $mask_index));

//                         w
//                     });
//                 }

//                 fn change_detector_is_enabled(&self) -> bool
//                 {
//                     let dp = unsafe { Peripherals::steal() };
//                     (dp.EXINT.$mask.read().pcint().bits() & (1 << $mask_index)) != 0
//                 }

//                 fn set_change_callback(&mut self, f: fn(is_high: bool))
//                 {
//                     interrupt::free(|_| unsafe
//                     {
//                         $static = Some(f);
//                     })
//                 }

//                 fn remove_change_callback(&mut self)
//                 {
//                     interrupt::free(|_| unsafe
//                     {
//                         $static = None;
//                     })
//                 }
//             }
//         )+
//     };
// }

// pin_change_impl!
// {
//     impl for port::PB0 { static PB0_CHANGE_CALLBACK; pcmsk0 << 0; pcie << 0 }
//     impl for port::PB1 { static PB1_CHANGE_CALLBACK; pcmsk0 << 1; pcie << 0 }
//     impl for port::PB2 { static PB2_CHANGE_CALLBACK; pcmsk0 << 2; pcie << 0 }
//     impl for port::PB3 { static PB3_CHANGE_CALLBACK; pcmsk0 << 3; pcie << 0 }
//     impl for port::PB4 { static PB4_CHANGE_CALLBACK; pcmsk0 << 4; pcie << 0 }
//     impl for port::PB5 { static PB5_CHANGE_CALLBACK; pcmsk0 << 5; pcie << 0 }
//     impl for port::PB6 { static PB6_CHANGE_CALLBACK; pcmsk0 << 6; pcie << 0 }
//     impl for port::PB7 { static PB7_CHANGE_CALLBACK; pcmsk0 << 7; pcie << 0 }

//     impl for port::PC0 { static PC0_CHANGE_CALLBACK; pcmsk1 << 0; pcie << 1 }
//     impl for port::PC1 { static PC1_CHANGE_CALLBACK; pcmsk1 << 1; pcie << 1 }
//     impl for port::PC2 { static PC2_CHANGE_CALLBACK; pcmsk1 << 2; pcie << 1 }
//     impl for port::PC3 { static PC3_CHANGE_CALLBACK; pcmsk1 << 3; pcie << 1 }
//     impl for port::PC4 { static PC4_CHANGE_CALLBACK; pcmsk1 << 4; pcie << 1 }
//     impl for port::PC5 { static PC5_CHANGE_CALLBACK; pcmsk1 << 5; pcie << 1 }
//     impl for port::PC6 { static PC6_CHANGE_CALLBACK; pcmsk1 << 6; pcie << 1 }

//     impl for port::PD0 { static PD0_CHANGE_CALLBACK; pcmsk2 << 0; pcie << 2 }
//     impl for port::PD1 { static PD1_CHANGE_CALLBACK; pcmsk2 << 1; pcie << 2 }
//     impl for port::PD2 { static PD2_CHANGE_CALLBACK; pcmsk2 << 2; pcie << 2 }
//     impl for port::PD3 { static PD3_CHANGE_CALLBACK; pcmsk2 << 3; pcie << 2 }
//     impl for port::PD4 { static PD4_CHANGE_CALLBACK; pcmsk2 << 4; pcie << 2 }
//     impl for port::PD5 { static PD5_CHANGE_CALLBACK; pcmsk2 << 5; pcie << 2 }
//     impl for port::PD6 { static PD6_CHANGE_CALLBACK; pcmsk2 << 6; pcie << 2 }
//     impl for port::PD7 { static PD7_CHANGE_CALLBACK; pcmsk2 << 7; pcie << 2 }
// }

// macro_rules! pin_change_impl
// {
//     (
//         $(
//             #[avr_device::interrupt(atmega328p)]
//             fn $pc:ident()
//             {
//                 $(
//                     $last:ident => 
//                 )+
//             }
//         )+
//     ) =>
//     {

//     }
// }

// static mut LAST_PCINT0_PIN_VALUES: u8 = 0xFF;

// #[avr_device::interrupt(atmega328p)]
// unsafe fn PCINT0()
// {
//     let dp = Peripherals::steal();
//     let current_pin_values = dp.PORTB.portb.read().bits();

//     let changes = current_pin_values

//                     dp.EXINT.pcifr.read().
// }

// #[avr_device::interrupt(atmega328p)]
// unsafe fn PCINT1() { _pcint() }

// #[avr_device::interrupt(atmega328p)]
// unsafe fn PCINT2() { _pcint() }