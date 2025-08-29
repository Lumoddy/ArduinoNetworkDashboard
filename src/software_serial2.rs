use arduino_hal::{hal::port, pac, port::{mode, Pin, PinOps}, Peripherals};
use avr_device::interrupt;
use core::{array, cmp, convert::Infallible, mem};
use crate::undo::{self, Undo};

// MARK: Reader
pub struct SoftSerialReaderPin<PIN>
{
    _reader_index: u8,
    _pin: undo::Undo<
        Pin<mode::Input<mode::PullUp>, PIN>,
        Pin<mode::Input<mode::PullUp>, port::Dynamic>>,
}

struct _SoftwareSerialReaderState
{
    pub pin: Pin<mode::Input<mode::PullUp>, port::Dynamic>,
    pub pin_was_high: bool,
    pub bit_accumulate: u8,
    pub bit_progress: u8,
    pub buffer: [u8; 64],
    pub buffer_head: u8,
    pub buffer_tail: u8,
    pub cycles_left_after_previous_overflow: u32,
    pub baudrate: u32,
    pub cycles_per_read: u32,
    pub one_and_half_cycles_per_read: u32,
}

/// Implementation detail. Adds `into_soft_serial_reader()` to `Pin<...>`.
pub trait IntoSoftSerialReaderPin: Sized
{
    type PIN: PinOps<Dynamic = port::Dynamic> + PCINTPin;

    fn into_soft_serial_reader(self, baudrate: u32)
        -> Result<SoftSerialReaderPin<Self::PIN>, Self>;
}

impl<MODE: mode::Io, PIN: PinOps<Dynamic = port::Dynamic> + PCINTPin>
    IntoSoftSerialReaderPin for Pin<MODE, PIN>
{
    type PIN = PIN;

    fn into_soft_serial_reader(self, baudrate: u32)
        -> Result<SoftSerialReaderPin<Self::PIN>, Self>
    {
        unsafe
        {
            #[allow(static_mut_refs)]
            let config = _CONFIG
                .as_mut()
                .expect(
                    "Software serial interrupt cannot be updated before its \
                    initialized.");

            for i in 0..config.reader_states.len()
            {
                let None = config.reader_states[i] else { continue };

                let cycles_per_read
                    = _CYCLES_PER_SECOND
                    / (baudrate * _PRESCALER.value() as u32);
                let pin = self.into_pull_up_input();
                let (pin, undo) = Undo::change(pin, Pin::downgrade);
                config.reader_states[i] = Some(_SoftwareSerialReaderState
                {
                    pin_was_high: pin.is_high(),
                    pin,

                    bit_accumulate: 0,
                    bit_progress: 0,
                    buffer: [0; 64],
                    buffer_head: 0,
                    buffer_tail: 0,
                    cycles_left_after_previous_overflow: 0,

                    baudrate,
                    cycles_per_read,
                    one_and_half_cycles_per_read: (cycles_per_read * 3) / 2,
                });

                PIN::_set_pcint();
                return Ok(SoftSerialReaderPin
                {
                    _pin: undo,
                    _reader_index: i as u8,
                });
            }

            Err(self)
        }
    }
}

impl<PIN: PinOps + PCINTPin> SoftSerialReaderPin<PIN>
{
    pub fn into_pin(self) -> Pin<mode::Input<mode::PullUp>, PIN>
    {
        unsafe
        {
            #[allow(static_mut_refs)]
            let config = _CONFIG
                .as_mut()
                .expect(
                    "Software serial interrupt cannot be updated before its \
                    initialized.");

            match mem::replace(
                &mut config.reader_states[self._reader_index as usize],
                None)
            {
                Some(reader) =>
                {
                    PIN::_clear_pcint();
                    self._pin.undo(reader.pin)
                },
                None => unreachable!(),
            }
        }
    }

    pub fn baudrate(&self) -> u32
    {
        #[allow(static_mut_refs)]
        let Some(config) = (unsafe { &mut _CONFIG }) else
        {
            panic!(
                "Software serial interrupt cannot be updated before its \
                initialized.");
        };

        config
            .reader_states[self._reader_index as usize]
            .as_ref()
            .unwrap()
            .baudrate
    }

    pub fn read(&self) -> Result<u8, nb::Error<Infallible>>
    {
        #[allow(static_mut_refs)]
        let Some(config) = (unsafe { &mut _CONFIG }) else
        {
            panic!(
                "Software serial interrupt cannot be updated before its \
                initialized.");
        };

        let reader = config
            .reader_states[self._reader_index as usize]
            .as_mut()
            .unwrap();

        if reader.buffer_tail == reader.buffer_head
        { return Err(nb::Error::WouldBlock) }

        let byte = reader.buffer[reader.buffer_tail as usize];
        reader.buffer_tail += 1;
        Ok(byte)
    }

    pub unsafe fn buffer_empty(&self) -> bool
    {
        #[allow(static_mut_refs)]
        let Some(config) = &mut _CONFIG else
        {
            panic!(
                "Software serial interrupt cannot be updated before its \
                initialized.");
        };

        let reader = config
            .reader_states[self._reader_index as usize]
            .as_ref()
            .unwrap();

        reader.buffer_head == reader.buffer_tail
    }

    pub unsafe fn buffer_count(&self) -> u8
    {
        #[allow(static_mut_refs)]
        let Some(config) = &mut _CONFIG else
        {
            panic!(
                "Software serial interrupt cannot be updated before its \
                initialized.");
        };

        let reader = config
            .reader_states[self._reader_index as usize]
            .as_ref()
            .unwrap();

        if reader.buffer_head < reader.buffer_tail
        {
            reader.buffer_head + reader.buffer.len() as u8 - reader.buffer_tail
        }
        else
        {
            reader.buffer_head - reader.buffer_tail
        }
    }
}

// MARK: Writer
pub struct SoftSerialWriterPin<PIN>
{
    _writer_index: u8,
    _pin: undo::Undo<
        Pin<mode::Output, PIN>,
        Pin<mode::Output, port::Dynamic>>,
}

struct _SoftwareSerialWriterState
{
    pub pin: Pin<mode::Output, port::Dynamic>,
    pub current_byte: u8,
    pub bit_progress: u8,
    pub buffer: [u8; 64],
    pub buffer_head: u8,
    pub buffer_tail: u8,
    pub cycles_left_after_previous_overflow: u32,
    pub baudrate: u32,
    pub cycles_per_read: u32,
}

/// Implementation detail. Adds `into_soft_serial_writer()` to `Pin<...>`.
pub trait IntoSoftSerialWriterPin: Sized
{
    type PIN: PinOps<Dynamic = port::Dynamic> + PCINTPin;

    fn into_soft_serial_writer(self, baudrate: u32)
        -> Result<SoftSerialWriterPin<Self::PIN>, Self>;
}

impl<MODE: mode::Io, PIN: PinOps<Dynamic = port::Dynamic> + PCINTPin>
    IntoSoftSerialWriterPin for Pin<MODE, PIN>
{
    type PIN = PIN;

    fn into_soft_serial_writer(self, baudrate: u32)
        -> Result<SoftSerialWriterPin<Self::PIN>, Self>
    {
        unsafe
        {
            #[allow(static_mut_refs)]
            let config = _CONFIG
                .as_mut()
                .expect(
                    "Software serial interrupt cannot be updated before its \
                    initialized.");

            for i in 0..config.writer_states.len()
            {
                let None = config.writer_states[i] else { continue };

                let cycles_per_read
                    = _CYCLES_PER_SECOND
                    / (baudrate * _PRESCALER.value() as u32);
                let pin = self.into_output();
                let (pin, undo) = Undo::change(pin, Pin::downgrade);
                config.writer_states[i] = Some(_SoftwareSerialWriterState
                {
                    pin,

                    current_byte: 0,
                    bit_progress: 0,
                    buffer: [0; 64],
                    buffer_head: 0,
                    buffer_tail: 0,
                    cycles_left_after_previous_overflow: 0,

                    baudrate,
                    cycles_per_read,
                });

                return Ok(SoftSerialWriterPin
                {
                    _pin: undo,
                    _writer_index: i as u8,
                });
            }

            Err(self)
        }
    }
}

impl<PIN: PinOps + PCINTPin> SoftSerialWriterPin<PIN>
{
    pub fn into_pin(self) -> Pin<mode::Output, PIN>
    {
        unsafe
        {
            #[allow(static_mut_refs)]
            let config = _CONFIG
                .as_mut()
                .expect(
                    "Software serial interrupt cannot be updated before its \
                    initialized.");

            match mem::replace(
                &mut config.writer_states[self._writer_index as usize],
                None)
            {
                Some(writer) => self._pin.undo(writer.pin),
                None => unreachable!(),
            }
        }
    }

    pub fn baudrate(&self) -> u32
    {
        unsafe
        {
            #[allow(static_mut_refs)]
            _CONFIG
                .as_ref()
                .expect(
                    "Software serial interrupt cannot be updated before its \
                    initialized.")
                .writer_states[self._writer_index as usize]
                .as_ref()
                .unwrap()
                .baudrate
        }
    }

    pub fn write(&mut self, byte: u8) -> Result<(), nb::Error<Infallible>>
    {
        #[allow(static_mut_refs)]
        let Some(config) = (unsafe { &mut _CONFIG }) else
        {
            panic!(
                "Software serial interrupt cannot be updated before its \
                initialized.");
        };

        let writer = config
            .writer_states[self._writer_index as usize]
            .as_mut()
            .unwrap();

        if writer.buffer_head + 1 == writer.buffer_tail
        { return Err(nb::Error::WouldBlock) }

        writer.buffer[writer.buffer_head as usize] = byte;

        if writer.buffer_head == writer.buffer.len() as u8 - 1
        {
            writer.buffer_head = 0;
        }
        else
        {
            writer.buffer_head += 1;
        }

        config.tc.timsk1.write(|w| w
            .ocie1a().set_bit());

        Ok(())
    }

    pub unsafe fn buffer_empty(&self) -> bool
    {
        #[allow(static_mut_refs)]
        let Some(config) = &mut _CONFIG else
        {
            panic!(
                "Software serial interrupt cannot be updated before its \
                initialized.");
        };

        let writer = config
            .writer_states[self._writer_index as usize]
            .as_ref()
            .unwrap();

        writer.buffer_head == writer.buffer_tail
    }

    pub unsafe fn buffer_count(&self) -> u8
    {
        #[allow(static_mut_refs)]
        let Some(config) = &mut _CONFIG else
        {
            panic!(
                "Software serial interrupt cannot be updated before its \
                initialized.");
        };

        let writer = config
            .writer_states[self._writer_index as usize]
            .as_ref()
            .unwrap();

        if writer.buffer_head < writer.buffer_tail
        {
            writer.buffer_head + writer.buffer.len() as u8 - writer.buffer_tail
        }
        else
        {
            writer.buffer_head - writer.buffer_tail
        }
    }
}

// MARK: Config
#[allow(unused)]
enum _Prescaler
{
    None,
    By8,
    By64,
    By256,
    By1024,
}

impl _Prescaler
{
    pub const fn cs_bits(&self) -> u8
    {
        match self
        {
            _Prescaler::None => 0b001,
            _Prescaler::By8 => 0b010,
            _Prescaler::By64 => 0b011,
            _Prescaler::By256 => 0b100,
            _Prescaler::By1024 => 0b101,
        }
    }

    pub const fn value(&self) -> u16
    {
        match self
        {
            _Prescaler::None => 1,
            _Prescaler::By8 => 8,
            _Prescaler::By64 => 64,
            _Prescaler::By256 => 256,
            _Prescaler::By1024 => 1024,
        }
    }
}

const _CYCLES_PER_SECOND: u32 = 16000000; // Arduino Uno
const _PRESCALER: _Prescaler = _Prescaler::By64;

struct _Config
{
    tc: pac::TC1,
    reader_states: [Option<_SoftwareSerialReaderState>; 4],
    writer_states: [Option<_SoftwareSerialWriterState>; 4],
}

static mut _CONFIG: Option<_Config> = None;

// MARK: Init
pub fn init() -> Result<(), ()>
{
    #[allow(static_mut_refs)]
    match unsafe { &_CONFIG }
    {
        Some(_) => Err(()),
        None =>
        {
            let dp = unsafe { Peripherals::steal() };

            // https://www.arxterra.com/11-atmega328p-external-interrupts/#External_Interrupts

            unsafe { interrupt::enable() };

            dp.EXINT.pcicr.write(|w| w
                .pcie().bits(0b111));
            dp.EXINT.eicra.write(|w| w
                .isc0().bits(0b01));

            dp.TC1.timsk1.write(|w| w
                .ocie1a().clear_bit());
            dp.TC1.tccr1a.write(|w| w
                .wgm1().bits(0b__00));
            dp.TC1.tccr1b.write(|w| w
                .wgm1().bits(0b01__)
                .cs1().bits(_PRESCALER.cs_bits()));

            unsafe
            {
                Ok(_CONFIG = Some(_Config
                {
                    tc: dp.TC1,
                    reader_states: array::from_fn(|_| None),
                    writer_states: array::from_fn(|_| None),
                }))
            }
        },
    }
}

macro_rules! _request_ocr1a
{
    ($tc:expr => ($ocr1a:ident => $ocie1a:ident()), $value:expr) =>
    {
        if $tc.timsk1.read().$ocie1a().bit_is_set()
        {
            $tc.$ocr1a.write(|w| w.bits(cmp::min(
                $tc.$ocr1a.read().bits(),
                $value)));
        }
        else
        {
            $tc.timsk1.write(|w| w
                .$ocie1a().set_bit());

            $tc.$ocr1a.write(|w| w.bits(
                $value));
        }
    };
}

// MARK: Interrupts
#[interrupt(atmega328p)]
fn PCINT0() { _pcint(0) }

#[interrupt(atmega328p)]
fn PCINT1() { _pcint(1) }

#[interrupt(atmega328p)]
fn PCINT2() { _pcint(2) }

fn _pcint(_: u8)
{
    #[allow(static_mut_refs)]
    let Some(config) = (unsafe { &mut _CONFIG }) else
    {
        panic!(
            "Software serial interrupt cannot be updated before its \
             initialized.");
    };

    for entry in &mut config.reader_states
    {
        let Some(reader) = entry else { continue };

        if (config.tc.tcnt1.read().bits() as u32)
            < reader.cycles_left_after_previous_overflow
        { continue }

        let pin_is_high = reader.pin.is_high();
        match reader.bit_progress
        {
            0 if pin_is_high && !reader.pin_was_high =>
            {
                reader.cycles_left_after_previous_overflow
                    = config.tc.tcnt1.read().bits() as u32
                    + reader.one_and_half_cycles_per_read;

                if reader.cycles_left_after_previous_overflow <= 0xFFFF
                {
                    _request_ocr1a!(
                        config.tc => (ocr1a => ocie1a()),
                        reader.cycles_left_after_previous_overflow as u16);
                }

                reader.bit_progress = 1;
            },
            1..=8 => (),
            9 => (),
            _ => unreachable!(),
        }
        reader.pin_was_high = pin_is_high;
    }
}

#[interrupt(atmega328p)]
fn TIMER1_OVF()
{
    #[allow(static_mut_refs)]
    let Some(config) = (unsafe { &mut _CONFIG }) else
    {
        panic!(
            "Software serial interrupt cannot be updated before its \
             initialized.");
    };

    config.tc.timsk1.write(|w| w
        .ocie1a().clear_bit());

    for entry in &mut config.reader_states
    {
        let Some(reader) = entry else { continue };

        if reader.cycles_left_after_previous_overflow > 0xFFFF
        {
            reader.cycles_left_after_previous_overflow -= 0x10000;

            if reader.cycles_left_after_previous_overflow <= 0xFFFF
            {
                _request_ocr1a!(
                    config.tc => (ocr1a => ocie1a()),
                    reader.cycles_left_after_previous_overflow as u16);
            }
        }
    }

    for entry in &mut config.writer_states
    {
        let Some(writer) = entry else { continue };

        if writer.cycles_left_after_previous_overflow > 0xFFFF
        {
            writer.cycles_left_after_previous_overflow -= 0x10000;

            if writer.cycles_left_after_previous_overflow <= 0xFFFF
            {
                _request_ocr1a!(
                    config.tc => (ocr1a => ocie1a()),
                    writer.cycles_left_after_previous_overflow as u16);
            }
        }
    }
}

#[interrupt(atmega328p)]
fn TIMER1_COMPA()
{
    #[allow(static_mut_refs)]
    let Some(config) = (unsafe { &mut _CONFIG }) else
    {
        panic!(
            "Software serial interrupt cannot be updated before its \
             initialized.");
    };

    let tcnt_value = config.tc.tcnt1.read().bits();

    config.tc.timsk1.write(|w| w
        .ocie1a().clear_bit());

    for entry in &mut config.reader_states
    {
        let Some(reader) = entry else { continue };

        if (tcnt_value as u32) < reader.cycles_left_after_previous_overflow
        { continue }

        match reader.bit_progress
        {
            0 => (),
            1..=8 =>
            {
                if reader.pin.is_high()
                {
                    reader.bit_accumulate |= 1 << reader.bit_progress;
                }

                reader.cycles_left_after_previous_overflow
                    = tcnt_value as u32
                    + reader.cycles_per_read;

                if reader.cycles_left_after_previous_overflow <= 0xFFFF
                {
                    _request_ocr1a!(
                        config.tc => (ocr1a => ocie1a()),
                        reader.cycles_left_after_previous_overflow as u16);
                }

                reader.bit_progress += 1;
            },
            9 =>
            {
                if reader.buffer_head == reader.buffer.len() as u8 - 1
                {
                    if reader.buffer_tail != 0
                    {
                        reader.buffer_head = 0;

                        reader.buffer[reader.buffer_head as usize] = mem::replace(
                            &mut reader.bit_accumulate,
                            0);
                    }
                }
                else
                {
                    if reader.buffer_head + 1 != reader.buffer_tail
                    {
                        reader.buffer_head += 1;

                        reader.buffer[reader.buffer_head as usize] = mem::replace(
                            &mut reader.bit_accumulate,
                            0);
                    }
                }

                reader.cycles_left_after_previous_overflow
                    = tcnt_value as u32
                    + reader.cycles_per_read;

                if reader.cycles_left_after_previous_overflow <= 0xFFFF
                {
                    _request_ocr1a!(
                        config.tc => (ocr1a => ocie1a()),
                        reader.cycles_left_after_previous_overflow as u16);
                }

                reader.bit_progress = 10;
            },
            10 =>
            {
                if reader.pin.is_high()
                {
                    reader.cycles_left_after_previous_overflow
                        = tcnt_value as u32
                        + reader.cycles_per_read;

                    if reader.cycles_left_after_previous_overflow <= 0xFFFF
                    {
                        _request_ocr1a!(
                            config.tc => (ocr1a => ocie1a()),
                            reader.cycles_left_after_previous_overflow as u16);
                    }

                    reader.bit_progress = 1;
                }
                else
                {
                    reader.bit_progress = 0;
                }
            },
            _ => unreachable!(),
        }
    }

    for entry in &mut config.writer_states
    {
        let Some(writer) = entry else { continue };

        match writer.bit_progress
        {
            0 =>
            {
                if writer.buffer_head == writer.buffer_tail
                { continue }

                writer.cycles_left_after_previous_overflow
                    = tcnt_value as u32
                    + writer.cycles_per_read;

                if writer.cycles_left_after_previous_overflow <= 0xFFFF
                {
                    _request_ocr1a!(
                        config.tc => (ocr1a => ocie1a()),
                        writer.cycles_left_after_previous_overflow as u16);
                }

                writer.current_byte = writer.buffer[writer.buffer_tail as usize];
                writer.buffer_tail += 1;

                writer.bit_progress = 1;
            },
            1..=8 =>
            {
                if (writer.current_byte >> (writer.bit_progress - 1)) != 0
                { writer.pin.set_high() }
                else
                { writer.pin.set_low() }

                writer.cycles_left_after_previous_overflow
                    = tcnt_value as u32
                    + writer.cycles_per_read;

                if writer.cycles_left_after_previous_overflow <= 0xFFFF
                {
                    _request_ocr1a!(
                        config.tc => (ocr1a => ocie1a()),
                        writer.cycles_left_after_previous_overflow as u16);
                }

                writer.bit_progress += 1;
            },
            9 =>
            {
                writer.pin.set_high();

                writer.cycles_left_after_previous_overflow
                    = tcnt_value as u32
                    + writer.cycles_per_read;

                if writer.cycles_left_after_previous_overflow <= 0xFFFF
                {
                    _request_ocr1a!(
                        config.tc => (ocr1a => ocie1a()),
                        writer.cycles_left_after_previous_overflow as u16);
                }

                writer.bit_progress = 0;
            },
            _ => unreachable!(),
        }
    }
}

// MARK: pcint_pin_impl
pub trait PCINTPin
{
    fn _set_pcint();
    fn _clear_pcint();
    fn _is_pcint_set() -> bool;
}

macro_rules! pcint_pin_impl
{
    (
        impl Pin<$port:ty>
        {
            $exint:ident.$mask:ident << $offset:expr
        }
    ) =>
    {
        impl PCINTPin for $port
        {
            fn _set_pcint()
            {
                let dp = unsafe { arduino_hal::Peripherals::steal() };
                dp.$exint.$mask.write(|w|
                {
                    w.bits(dp.$exint.$mask.read().bits() | (1 << $offset))
                });
            }

            fn _clear_pcint()
            {
                let dp = unsafe { arduino_hal::Peripherals::steal() };
                dp.$exint.$mask.write(|w|
                {
                    w.bits(dp.$exint.$mask.read().bits() & !(1 << $offset))
                });
            }

            fn _is_pcint_set() -> bool
            {
                let dp = unsafe { arduino_hal::Peripherals::steal() };
                (dp.$exint.$mask.read().bits() >> $offset) & 0x1 != 0
            }
        }
    };

    (
        impl Pin<$port:ty>
        {
            unsafe { $exint:ident.$mask:ident << $offset:expr }
        }
    ) =>
    {
        impl PCINTPin for $port
        {
            fn _set_pcint()
            {
                let dp = unsafe { arduino_hal::Peripherals::steal() };
                dp.$exint.$mask.write(|w| unsafe
                {
                    w.bits(dp.$exint.$mask.read().bits() | (1 << $offset))
                });
            }

            fn _clear_pcint()
            {
                let dp = unsafe { arduino_hal::Peripherals::steal() };
                dp.$exint.$mask.write(|w| unsafe
                {
                    w.bits(dp.$exint.$mask.read().bits() & !(1 << $offset))
                });
            }

            fn _is_pcint_set() -> bool
            {
                let dp = unsafe { arduino_hal::Peripherals::steal() };
                (dp.$exint.$mask.read().bits() >> $offset) & 0x1 != 0
            }
        }
    };

    (
        impl Pin<$first_port:ty> $first_contents:tt
        $(impl Pin<$port:ty> $contents:tt)+
    ) =>
    {
        pcint_pin_impl! { impl Pin<$first_port> $first_contents }
        $(pcint_pin_impl! { impl Pin<$port> $contents })+
    };
}

pcint_pin_impl!
{
    impl Pin<port::PB0> { EXINT.pcmsk0 << 0 }
    impl Pin<port::PB1> { EXINT.pcmsk0 << 1 }
    impl Pin<port::PB2> { EXINT.pcmsk0 << 2 }
    impl Pin<port::PB3> { EXINT.pcmsk0 << 3 }
    impl Pin<port::PB4> { EXINT.pcmsk0 << 4 }
    impl Pin<port::PB5> { EXINT.pcmsk0 << 5 }
    impl Pin<port::PB6> { EXINT.pcmsk0 << 6 }
    impl Pin<port::PB7> { EXINT.pcmsk0 << 7 }

    impl Pin<port::PC0> { unsafe { EXINT.pcmsk1 << 0 } }
    impl Pin<port::PC1> { unsafe { EXINT.pcmsk1 << 1 } }
    impl Pin<port::PC2> { unsafe { EXINT.pcmsk1 << 2 } }
    impl Pin<port::PC3> { unsafe { EXINT.pcmsk1 << 3 } }
    impl Pin<port::PC4> { unsafe { EXINT.pcmsk1 << 4 } }
    impl Pin<port::PC5> { unsafe { EXINT.pcmsk1 << 5 } }
    impl Pin<port::PC6> { unsafe { EXINT.pcmsk1 << 6 } }

    impl Pin<port::PD0> { EXINT.pcmsk2 << 0 }
    impl Pin<port::PD1> { EXINT.pcmsk2 << 1 }
    impl Pin<port::PD2> { EXINT.pcmsk2 << 2 }
    impl Pin<port::PD3> { EXINT.pcmsk2 << 3 }
    impl Pin<port::PD4> { EXINT.pcmsk2 << 4 }
    impl Pin<port::PD5> { EXINT.pcmsk2 << 5 }
    impl Pin<port::PD6> { EXINT.pcmsk2 << 6 }
    impl Pin<port::PD7> { EXINT.pcmsk2 << 7 }
}