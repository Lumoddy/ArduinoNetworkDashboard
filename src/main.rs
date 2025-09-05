#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![allow(static_mut_refs)]

// mod coder
mod panic_handler;
mod soft_serial;

use arduino_hal::{delay_ms, prelude::_unwrap_infallible_UnwrapInfallible};
use panic_handler::write_payload_and_panic;
use soft_serial::input::{SerialInput, SerialInputConfig, SerialInputError};
use soft_serial::output::{SerialOutput, SerialOutputConfig, SerialOutputError};
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

    dp.CPU.mcusr.modify(|r, w|
    {
        if r.borf().bit_is_set()
        {
            uwrite!(serial, "!Reset caused by brown out.\n").unwrap_infallible();

            w.borf().clear_bit();
        }
        else if r.extrf().bit_is_set()
        {
            uwrite!(serial, "!Reset caused by a manual reset.\n").unwrap_infallible();

            w.extrf().clear_bit();
        }
        else if r.porf().bit_is_set()
        {
            uwrite!(serial, "!Start caused by power on.\n").unwrap_infallible();

            w.porf().clear_bit();
        }
        else if r.wdrf().bit_is_set()
        {
            uwrite!(serial, "!Reset caused by watchdog.\n").unwrap_infallible();

            w.wdrf().clear_bit();
        }
        else
        {
            uwrite!(serial, "!Started for the first time.\n").unwrap_infallible();
        }

        w
    });

    uwrite!(serial, "!Initializing Tests...\n").unwrap_infallible();

    soft_serial::init_service(dp.TC1, dp.EXINT);

    let Ok(mut serial_input) = SerialInput::init(SerialInputConfig
    {
        pin: pins.d12.into_pull_up_input(),
        baudrate: 50,
        inverse_signal: false,
    })
    else { unreachable_payload!() };

    let Ok(mut serial_output) = SerialOutput::init(SerialOutputConfig
    {
        pin: pins.d13.into_output_high(),
        baudrate: 50,
        inverse_signal: false,
    })
    else { unreachable_payload!() };

    uwrite!(serial, "!Starting Tests...\n").unwrap_infallible();

    delay_ms(2000);

    loop
    {
        avr_device::asm::wdr();

        uwrite!(serial, "! {{\n").unwrap_infallible();

        match uwrite!(serial_output, "5")
        {
            Ok(()) => (),
            Err(SerialOutputError::TooManyParallel) =>
            {
                write_payload_and_panic(|w|
                {
                    uwrite!(w, "SerialInputError::TooManyParallel")?;

                    Ok(())
                });
            },
        }

        match serial_input.take_err()
        {
            Some(SerialInputError::TooManyParallel) =>
            {
                write_payload_and_panic(|w|
                {
                    uwrite!(w, "SerialInputError::TooManyParallel: \"")?;

                    for byte in serial_input.continuous_read()
                    {
                        w.write_byte(byte);
                    }

                    uwrite!(w, "\"\n")?;

                    Ok(())
                });
            },
            Some(SerialInputError::IncompleteData) =>
            {
                write_payload_and_panic(|w|
                {
                    uwrite!(w, "SerialInputError::IncompleteData: \"")?;

                    for byte in serial_input.continuous_read()
                    {
                        w.write_byte(byte);
                    }

                    uwrite!(w, "\"\n")?;

                    Ok(())
                });
            },
            None =>
            {
                uwrite!(serial, "Read: \"").unwrap_infallible();

                for byte in serial_input.continuous_read()
                {
                    serial.write_byte(byte);
                }

                uwrite!(serial, "\"\n").unwrap_infallible();
            }
        }

        uwrite!(serial, "! }}\n").unwrap_infallible();

        delay_ms(2000);
    }
}