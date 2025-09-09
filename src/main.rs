#![no_std]
#![no_main]
#![allow(static_mut_refs)]
#![feature(abi_avr_interrupt)]

mod panic_handler;
mod nbt;
mod common;

use core::convert::Infallible;

use arduino_hal::{delay_ms};
use nbt::writer::{WriteCompound, WriteName, Write};
use panic_handler::UnwrapPayload;
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

    delay_ms(1000);

    uwrite!(serial, "NBT: [\n").unwrap_payload();

    let mut raw_writer = nbt::writer::new(
        nbt::Endian::Little,
        |byte| Ok::<_, Infallible>(
        {
            if (b'a'..=b'z').contains(&byte)
            {
                serial.write_byte(b'\'');
                serial.write_byte(byte);
                serial.write_byte(b' ');
            }
            else
            {
                uwrite!(serial, "{:02X} ", byte)?;
            }
        }));

    let mut writer = raw_writer.safe();

    writer = || -> Result<_, _>
    {
        writer
            .compound()?.name("")?
                .string()?.name("type")?.write("test")?
                .int()?.name("value")?.write(420)?
                .end()
    }()
    .unwrap_payload();

    drop(writer);

    uwrite!(serial, "\n]\n").unwrap_payload();

    // let mut reader = nbt::reader::ClosureReader::new(
    //     nbt::Endian::Little,
    //     |byte| Ok::<_, Infallible>(
    //     {
    //         if (b'a'..=b'z').contains(&byte)
    //         {
    //             serial.write_byte(b'\'');
    //             serial.write_byte(byte);
    //             serial.write_byte(b' ');
    //         }
    //         else
    //         {
    //             uwrite!(serial, "{:02X} ", byte)?;
    //         }
    //     }));

    // reader = || -> Result<_, _>
    // {
    //     match reader.entry()
    //     {
    //         TypeVarant
    //     }
    //         .compound()?.name("")?
    //             .string()?.name("type")?.write("test")?
    //             .int()?.name("value")?.write(420)?
    //             .end()
    // }()
    // .unwrap_payload();

    // drop(reader);

    // nbt::serialize_byte_array_be::<Infallible>(&[6i8].as_slice(), |byte| Ok(serial.write_byte(byte))).unwrap_infallible();

    loop { }
}

// struct Test
// {

// }

// impl<'a> nbt::SerializeAsByteArray<'a> for Test
// {
//     type Error = Infallible;

//     fn serialize<W: nbt::CompoundSerializerWrite>(self, writer: W)
//         -> Result<Result<W::Parent, Infallible>, W::Error>
//     {
//         let a = writer
//             .compound()?.name("")?
//                 .byte()?.name("byte")?.value(5)?
//                 .string()?.name("name")?.value("something")?
//                 .byte()?.name("")?.value(1)?
//                 .compound()?.name("group")?
//                     .byte_array()?.name("blob")?.of(&[1, 2, 3, 4])?
//                     .end()?
//                 .end()?
//             .end()?;

//         Ok(Ok(a))
//     }
// }