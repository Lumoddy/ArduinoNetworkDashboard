#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![allow(static_mut_refs)]

// mod coder
mod panic_handler;
mod soft;
mod format;

use core::{convert::Infallible, mem};

use arduino_hal::{delay_ms, prelude::_unwrap_infallible_UnwrapInfallible};
use avr_device::interrupt;
use heapless::Vec;
use soft::writer::WriterConfig;
use soft::{exint, reader, writer};

use soft::tc1;

// use crate::software_serial2::{IntoSoftSerialReaderPin, IntoSoftSerialWriterPin};
// use software_serial2::*;

static mut TC1_SCHEDULER_ALLOCATION: tc1::SchedulerAllocation<16>
    = tc1::SchedulerAllocation::new();

static mut EXINT_SCHEDULER_ALLOCATION: exint::SchedulerAllocation<16>
    = exint::SchedulerAllocation::new();

static mut READER_BUFFER: heapless::Deque<u8, 32> = heapless::Deque::new();
static mut WRITER_BUFFER: heapless::Deque<u8, 32> = heapless::Deque::new();

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
            for byte in b"!Reset caused by brown out.\n"
            { serial.write_byte(*byte) }

            w.borf().clear_bit();
        }
        else if r.extrf().bit_is_set()
        {
            for byte in b"!Reset caused by a manual reset.\n"
            { serial.write_byte(*byte) }

            w.extrf().clear_bit();
        }
        else if r.porf().bit_is_set()
        {
            for byte in b"!Start caused by power on.\n"
            { serial.write_byte(*byte) }

            w.porf().clear_bit();
        }
        else if r.wdrf().bit_is_set()
        {
            for byte in b"!Reset caused by watchdog.\n"
            { serial.write_byte(*byte) }

            w.wdrf().clear_bit();
        }
        else
        {
            for byte in b"!Started for the first time.\n"
            { serial.write_byte(*byte) }
        }

        w
    });

    for byte in b"!Initializing Tests...\n"
    { serial.write_byte(*byte) }

    let Ok(tc1_scheduler) = tc1::Scheduler::init(tc1::SchedulerConfig
    {
        tc: dp.TC1,
        allocation: unsafe { &mut TC1_SCHEDULER_ALLOCATION },
    })
    else { unreachable!() };

    let Ok(exint_scheduler) = exint::Scheduler::init(exint::SchedulerConfig
    {
        exint: dp.EXINT,
        allocation: unsafe { &mut EXINT_SCHEDULER_ALLOCATION },
    })
    else { unreachable!() };

    for byte in b"!Starting Tests...\n"
    { serial.write_byte(*byte) }

    let Ok(mut serial_writer) = soft::writer::init(
        pins.d13.into_output(),
        soft::writer::WriterConfig
        {
            baudrate: 9600,
            tc1: &tc1_scheduler,
            buffer: unsafe { &mut WRITER_BUFFER },
            inverse_voltage: false,
        })
    else { panic_payload!(str: b"Failed to init serial writer") };

    let Ok(mut serial_reader) = soft::reader::init(
        pins.d12.into_pull_up_input(),
        soft::reader::ReaderConfig
        {
            baudrate: 9600,
            tc1: &tc1_scheduler,
            exint: &exint_scheduler,
            buffer: unsafe { &mut READER_BUFFER },
            inverse_voltage: false,
        })
    else { panic_payload!(str: b"Failed to init serial writer") };

    loop
    {
        for byte in b"! ->\n"
        { serial.write_byte(*byte) }

        if serial_writer.is_writing() || serial_reader.is_reading()
        {
            serial.write_byte(b'!');
            delay_ms(500);

            while serial_writer.is_writing() || serial_reader.is_reading()
            {
                serial.write_byte(b'.');
                delay_ms(500);
            }

            serial.write_byte(b'\n');
        }

        delay_ms(1000);

        interrupt::free(|_| unsafe
        {
            for byte in b"This has been sent through the soft serial writer.\n"
            {
                let Ok(()) = WRITER_BUFFER.push_front(*byte)
                else { break };
            }
        });

        match serial_writer.start_write_now()
        {
            Ok(()) => (),
            Err(writer::WriteError::AlreadyWriting) =>
            {
                panic_payload!(str: b"WriteError::AlreadyWriting");
            },
            Err(writer::WriteError::NothingToWrite) =>
            {
                panic_payload!(str: b"WriteError::NothingToWrite");
            },
            Err(writer::WriteError::TaskQueueFull) =>
            {
                panic_payload!(str: b"WriteError::TaskQueueFull");
            },
        }

        avr_device::asm::wdr();

        delay_ms(1000);
    }
}

// #[avr_device::interrupt(atmega328p)]
// unsafe fn TIMER1_OVF()
// {
//     PRINT.push(b'V').unwrap();
// }

// #[avr_device::interrupt(atmega328p)]
// unsafe fn TIMER1_COMPA()
// {
//     PRINT.push(b'C').unwrap();
// }

        // dp.TC1.tccr1b.modify(|r, w| w.cs1().variant());

    // for byte in b"starting...\n"
    // {
    //     serial.write_byte(*byte);
    // }

    // software_serial2::init().unwrap();

    // let Ok(mut serial_writer) = pins.d13.into_soft_serial_writer(2) else
    // { unreachable!() };

    // let Ok(serial_reader) = pins.d12.into_soft_serial_reader(2) else
    // { unreachable!() };

    // for byte in b"started...\n"
    // {
    //     serial.write_byte(*byte);
    // }

    // dp.EXINT.pcmsk0.write(|w| w.pcint().bits(0b00000001));
    // dp.EXINT.pcicr.write(|w| w.pcie().bits(0b111));
    // dp.EXINT.eicra.write(|w| w.isc0().bits(0b11));

    // dp.TC1.timsk1.write(|w| w.ocie1a().set_bit());
    // dp.TC1.tccr1a.write(|w| w.wgm1().bits(0b00));
    // dp.TC1.tccr1b.write(|w| w.wgm1().bits(0b01).cs1().bits(0b100));
    // dp.TC1.ocr1a.write(|w| w.bits(60000));

    // unsafe
    // {
    //     avr_device::interrupt::enable();
    //     led_pin = Some(pins.d13.into_output().downgrade());
    //     tcor = Some(&dp.TC1.ocr1a);
    // }

    // let _ = pins.d8.into_pull_up_input();

    // loop
    // {
    //     for byte in b"writing...\n"
    //     {
    //         serial.write_byte(*byte);
    //     }

    //     loop { }

    //     serial_writer.write(0b00110100).unwrap();
    //     delay_ms(5000);

    //     for byte in b"reading...\n"
    //     {
    //         serial.write_byte(*byte);
    //     }

    //     delay_ms(1000);
    //     let mut led_pin = serial_writer.into_pin();
    //     if serial_reader.read().is_ok()
    //     {
    //         for byte in b"good\n"
    //         {
    //             serial.write_byte(*byte);
    //         }

    //         led_pin.set_high();
    //         delay_ms(100);
    //         led_pin.set_low();
    //     }
    //     else
    //     {
    //         for byte in b"bad\n"
    //         {
    //             serial.write_byte(*byte);
    //         }
    //     }
    //     delay_ms(1000);

    //     if let Ok(new_serial_writer) = led_pin.into_soft_serial_writer(2)
    //     {
    //         serial_writer = new_serial_writer;
    //     }
    //     else
    //     { unreachable!() };
    // }
