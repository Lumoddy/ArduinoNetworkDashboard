#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![allow(static_mut_refs)]

// mod coder
mod panic_handler;
// mod software_serial1;
// mod software_serial2;
mod soft_serial;
// mod cpp_SoftwareSerial;
mod undo;
mod base;
mod format;

use core::{convert::Infallible, mem};

use arduino_hal::{delay_ms, prelude::_unwrap_infallible_UnwrapInfallible};
use avr_device::interrupt;
use heapless::Vec;

use crate::soft_serial::tc1::{self, SchedulerAllocation, SchedulerTaskContext};

// use crate::software_serial2::{IntoSoftSerialReaderPin, IntoSoftSerialWriterPin};
// use software_serial2::*;

static mut TC1_SCHEDULER_ALLOCATION: SchedulerAllocation<16>
    = SchedulerAllocation::new();

static mut PRINT: Vec<u8, 256> = Vec::new();
static mut PRINT_NUMBER: Option<u64> = None;

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

    let Ok(scheduler) = tc1::Scheduler::init(
        dp.TC1,
        unsafe { &mut TC1_SCHEDULER_ALLOCATION }) else { unreachable!() };

    for byte in b"!Starting Tests...\n"
    { serial.write_byte(*byte) }

    unsafe fn append_a(context: SchedulerTaskContext)
    {
        unwrap_payload!(PRINT.push(b'|'));

        unwrap_payload!(context.scheduler.schedule_task_absolute(
            0,
            context.cycles_since_init + 10000,
            append_a));
    }

    unsafe fn append_b(context: SchedulerTaskContext)
    {
        unwrap_payload!(PRINT.push(b'-'));

        unwrap_payload!(context.scheduler.schedule_task_absolute(
            0,
            context.cycles_since_init + 2000,
            append_b));
    }

    unsafe fn append_c(context: SchedulerTaskContext)
    {
        unwrap_payload!(PRINT.push(b'('));
        unwrap_payload!(PRINT.push(b')'));

        unwrap_payload!(context.scheduler.schedule_task_absolute(
            0,
            context.cycles_since_init + 1000000,
            append_c));
    }

    unsafe fn append_d(context: SchedulerTaskContext)
    {
        PRINT_NUMBER = Some(context.cycles_since_init);

        unwrap_payload!(context.scheduler.schedule_task_absolute(
            0,
            context.cycles_since_init + 10000000,
            append_d));
    }

    unwrap_payload!(scheduler.schedule_task_absolute(0, 10000, append_a));
    unwrap_payload!(scheduler.schedule_task_absolute(0, 10000, append_b));
    unwrap_payload!(scheduler.schedule_task_absolute(0, 100000, append_c));
    unwrap_payload!(scheduler.schedule_task_absolute(0, 1000000, append_d));

    // dp.TC1.tcnt1.write(|w| w.bits(0));
    // dp.TC1.tccr1a.write(|w| w
    //     .wgm1().bits(0b__00));
    // dp.TC1.tccr1b.write(|w| w
    //     .wgm1().bits(0b01__)
    //     .cs1().variant(arduino_hal::pac::tc1::tccr1b::CS1_A::PRESCALE_1024));
    // dp.TC1.timsk1.write(|w| w
    //     .ocie1a().set_bit()
    //     .toie1().set_bit());
    // dp.TC1.ocr1a.write(|w| w
    //     .bits(200));

    // unsafe
    // {
    //     interrupt::enable();
    // }

    loop
    {
        interrupt::free(|_| unsafe
        {
            let message = mem::replace(&mut PRINT, Vec::new());

            for byte in message
            { serial.write_byte(byte) }

            serial.write_byte(b'\n');
        });

        interrupt::free(|_| unsafe
        {
            if let Some(mut value) = mem::replace(&mut PRINT_NUMBER, None)
            {
                for byte in b"\n[ "
                { serial.write_byte(*byte) }

                let mut buffer = heapless::Vec::<u8, 16>::new();

                loop
                {
                    let digit = value % 10 as u64;
                    value /= 10 as u64;

                    buffer.push(b'0' + digit as u8);

                    if value == 0 { break };
                }

                buffer.reverse();
                for byte in buffer
                { serial.write_byte(byte) }

                for byte in b" ]\n"
                { serial.write_byte(*byte) }
            }
        });

        delay_ms(80);

        avr_device::asm::wdr();
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
