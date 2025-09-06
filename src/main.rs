#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![allow(static_mut_refs)]

mod panic_handler;
mod nbt;
mod common;

use core::convert::Infallible;

use common::serialize::{Serialize, SerializeResult};
use nbt::{CompoundContentWriter, CompoundNameWriter, NamedTagListWriter, Serializer, SimpleValueNameWriter, SimpleValueWriter};
use ufmt::uwrite;

#[arduino_hal::entry]
fn main() -> !
{
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::Usart::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(),
        arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(9600));

    let a = Serializer::new(Test { }, nbt::Endian::Little);
    _ = a.drain_infallible(|byte| serial.write_byte(byte));

    loop { }
}

struct Test
{

}

impl nbt::SerializeAsTag for Test
{
    type Error = Infallible;

    fn serialize<W: nbt::NamedTagListWriter>(self, writer: W)
        -> Result<Result<W::Return, Infallible>, W::Error>
    {
        Ok(Ok(writer
            .begin_compound()?.name("")?
                .int()?.name("A")?.value(1)?
                .byte()?.name("B")?.value(8)?
                .end_compound()?))
    }
}