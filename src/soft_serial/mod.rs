
pub mod input;
pub mod output;
pub mod pcint;
pub(self) mod interrupts;
pub(self) mod state;

pub use interrupts::init_service;

#[allow(non_camel_case_types)]
pub type ucycles = u64;

pub const SERIAL_BUFFER_CAPACITY: usize = 256;
pub const SERIAL_PIN_CAPACITY: usize = 2;