use core::mem::MaybeUninit;

use arduino_hal::{clock::Clock, port::{mode, Pin, PinOps}, simple_pwm::Prescaler, DefaultClock};

use crate::soft_serial::tc1;

struct SoftSerialReader<'s, PIN: PinOps>
{
    _pin: Pin<mode::Input<mode::PullUp>, PIN>,
    _scheduler: &'s tc1::Scheduler,
    _baud_cycles: u32,
    _last_bit_cycle_time: u64,
}

trait IntoSoftSerialReaderPin<'s, PIN: PinOps>
{
    fn into_soft_serial_reader(
        self,
        baudrate: u32,
        scheduler: &'s tc1::Scheduler) -> SoftSerialReader<'s, PIN>;
}

impl<'s, MODE: mode::Io, PIN: PinOps>
    IntoSoftSerialReaderPin<'s, PIN> for Pin<MODE, PIN>
{
    fn into_soft_serial_reader(
        self,
        baudrate: u32,
        scheduler: &'s tc1::Scheduler) -> SoftSerialReader<'s, PIN>
    {
        SoftSerialReader
        {
            _pin: self.into_pull_up_input(),
            _scheduler: scheduler,
            _baud_cycles: DefaultClock::FREQ / (baudrate * match tc1::PRESCALER
            {
                Prescaler::Direct => 1,
                Prescaler::Prescale8 => 8,
                Prescaler::Prescale64 => 64,
                Prescaler::Prescale256 => 256,
                Prescaler::Prescale1024 => 1024,
            }),
            _last_bit_cycle_time: 0,
        }
    }
}

impl<'s, PIN: PinOps> SoftSerialReader<'s, PIN>
{
    
}