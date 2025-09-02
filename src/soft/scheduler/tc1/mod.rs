use arduino_hal::simple_pwm::Prescaler;

mod allocation;
mod scheduler;
mod state;

pub use allocation::*;
pub use scheduler::*;
pub use state::*;

pub const PRESCALER: Prescaler = Prescaler::Prescale8;