#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![allow(static_mut_refs)]

mod panic_handler;
mod nbt;
mod common;

use core::convert::Infallible;

use common::serialize::Serialize;
use nbt::{CompoundSerializerWrite, CompoundSerializerWriteName, PrimitiveListSerializerWrite, PrimitiveListSerializerWriteName, PrimitiveSerializerWrite as _, PrimitiveSerializerWriteName as _, Serializer, TypePickerSerializerWrite};

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

impl nbt::SerializeAsCompoundTag for Test
{
    type Error = Infallible;

    fn serialize<W: nbt::CompoundSerializerWrite>(self, writer: W)
        -> Result<Result<W::Parent, Infallible>, W::Error>
    {
        let a = writer
            .compound()?.name("")?
                .byte()?.name("byte")?.value(5)?
                .string()?.name("name")?.value("something")?
                .byte()?.name("")?.value(1)?
                .compound()?.name("group")?
                    .byte_array()?.name("blob")?.of(&[1, 2, 3, 4])?
                    .end()?
                .end()?
            .end()?;

        Ok(Ok(a))
    }
}