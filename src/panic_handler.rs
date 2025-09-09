use core::convert::Infallible;

use arduino_hal::port::mode;
use arduino_hal::port::Pin;
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use arduino_hal::{delay_ms, pac::wdt::wdtcsr};
use avr_device::interrupt;
use ufmt::{uWrite, uwrite};

#[macro_export]
macro_rules! panic_payload
{
    ($($args:expr),+) =>
    {
        {
            $crate::panic_handler::write_payload_and_panic(|w| ufmt::uwrite!(w, $($args),+));
        }
    };
}

#[macro_export]
macro_rules! marker_panic_payload
{
    () =>
    {
        $crate::panic_payload!(
            "{}:{}",
            file!(),
            line!())
    };
}

#[macro_export]
macro_rules! unreachable_payload
{
    () =>
    {
        $crate::panic_payload!(
            "Encountered unreachable at {}:{}",
            file!(),
            line!())
    };
}

pub struct Payload
{
    _serial: arduino_hal::Usart<
        avr_device::atmega328p::USART0,
        Pin<mode::Input<mode::AnyInput>, arduino_hal::hal::port::PD0>,
        Pin<mode::Output, arduino_hal::hal::port::PD1>>,
}

impl Payload
{
    pub fn write_byte(&mut self, byte: u8) { self._serial.write_byte(byte) }
}

impl uWrite for Payload
{
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Infallible>
    {
        self._serial.write_str(s)
    }

    fn write_char(&mut self, c: char) -> Result<(), Infallible>
    {
        self._serial.write_char(c)
    }
}

pub trait ExpectPayload
{
    type Value;
    type Error;

    fn expect_payload(
        self,
        f: impl FnOnce(&mut Payload) -> Result<(), Infallible>) -> Self::Value;
}

impl<T, E> ExpectPayload for Result<T, E>
{
    type Value = T;
    type Error = E;

    fn expect_payload(
        self,
        f: impl FnOnce(&mut Payload) -> Result<(), Infallible>) -> T
    {
        match self
        {
            Self::Ok(value) => value,
            Self::Err(_) => write_payload_and_panic(f),
        }
    }
}

pub trait UnwrapPayload
{
    type Value;
    type Error;

    fn unwrap_payload(self) -> Self::Value;
}

impl<T, E> UnwrapPayload for Result<T, E>
{
    type Value = T;
    type Error = E;

    fn unwrap_payload(self) -> T
    {
        match self
        {
            Self::Ok(value) => value,
            Self::Err(_) => write_payload_and_panic(|w| uwrite!(
                w,
                "Unwrapped error at {}:{}",
                file!(),
                line!())),
        }
    }
}

static mut _PANIC_ALREADY_PRINTED: bool = false;

pub fn write_payload_and_panic(
    f: impl FnOnce(&mut Payload) -> Result<(), Infallible>) -> !
{
    interrupt::disable();

    unsafe
    {
        delay_ms(500);

        let dp = arduino_hal::Peripherals::steal();
        let pins = arduino_hal::pins!(dp);

        let mut payload = Payload
        {
            _serial: arduino_hal::Usart::new(
                dp.USART0,
                pins.d0,
                pins.d1.into_output(),
                arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(9600)),
        };

        uwrite!(payload._serial, "!Panic: ").unwrap_infallible();

        f(&mut payload).unwrap_infallible();

        uwrite!(payload._serial, "\n!Resetting...\n").unwrap_infallible();

        _PANIC_ALREADY_PRINTED = true;
    }

    panic!();
}

#[inline(never)]
#[panic_handler]
unsafe fn panic(_: &core::panic::PanicInfo) -> !
{
    let dp = arduino_hal::Peripherals::steal();

    if !_PANIC_ALREADY_PRINTED
    {
        interrupt::disable();

        delay_ms(500);

        let pins = arduino_hal::pins!(dp);

        let mut serial = arduino_hal::Usart::new(
            dp.USART0,
            pins.d0,
            pins.d1.into_output(),
            arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(9600));

        uwrite!(serial, "!Panic Unknown\n!Resetting...\n").unwrap_infallible();
    }

    delay_ms(1000);

    dp.CPU.mcusr.write(|w| w
        .wdrf().set_bit());
    dp.WDT.wdtcsr.write(|w| w
        .wde().set_bit()
        .wdph().variant(false)
        .wdpl().variant(wdtcsr::WDPL_A::CYCLES_64K));

    loop { }
}