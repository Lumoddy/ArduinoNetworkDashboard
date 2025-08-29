// use core::{array, mem::MaybeUninit};

// use arduino_hal::{clock::Clock, hal::port, port::{mode, Pin, PinOps}, DefaultClock, Peripherals};
// use avr_device::interrupt;
// use avr_device::interrupt::CriticalSection;

// pub const MAX_SERIAL_READERS: usize = 8;
// const _PRESCALER: _Prescaler = _Prescaler::By8;

// pub struct SoftSerialReader<PIN: PinOps<Dynamic = port::Dynamic>>
// {
//     _ghost_pin: MaybeUninit<Pin<mode::Input<mode::PullUp>, PIN>>,
//     _reader_index: u8,
// }

// pub trait IntoSoftSerialReader<MODE, PIN>: Sized
// where MODE: mode::Io, PIN: PinOps<Dynamic = port::Dynamic>
// {
//     fn into_soft_serial_reader(
//         self,
//         baudrate: u32,
//         read_callback: &'static dyn FnMut(u8, CriticalSection))
//         -> Result<SoftSerialReader<PIN>, Self>;
// }

// impl<MODE, PIN> IntoSoftSerialReader<MODE, PIN> for Pin<MODE, PIN>
// where MODE: mode::Io, PIN: PinOps<Dynamic = port::Dynamic>
// {
//     fn into_soft_serial_reader(
//         self,
//         baudrate: u32,
//         read_callback: &'static dyn FnMut(u8, CriticalSection))
//         -> Result<SoftSerialReader<PIN>, Self>
//     {
//         let Some(reader_system) = (unsafe { &mut _READER_SYSTEM })
//         else { return Err(self) };

//         for (i, slot) in reader_system._readers.iter_mut().enumerate()
//         {
//             let None = slot else { continue };

//             let pin = self.into_pull_up_input();

//             let result = SoftSerialReader::<PIN>
//             {
//                 _ghost_pin: MaybeUninit::new(
//                     unsafe { core::ptr::read(&raw const pin) }),
//                 _reader_index: i as u8,
//             };

//             interrupt::free(|_|
//             {
//                 *slot = Some(_SoftSerialReaderState
//                 {
//                     _pin_was_high: pin.is_high(),
//                     _pin: pin.downgrade(),
//                     _callback: read_callback,
//                     _byte_accumulate: 0,
//                     _progress: _SoftSerialReaderStateProgress::Waiting,
//                     _cycles_per_bit:
//                         (DefaultClock::FREQ * _PRESCALER.value() as u32)
//                         / baudrate,
//                     _cycles_per_one_and_a_half_bits:
//                         (DefaultClock::FREQ * _PRESCALER.value() as u32 * 3)
//                         / (baudrate * 2),
//                     _high_equals_one: false, // Low is 1 by default.
//                     _cycles_between_last_ovf_and_next_compa: todo!(),
//                 });
//             });

//             return Ok(result);
//         }

//         Err(self)
//     }
// }

// impl<PIN> SoftSerialReader<PIN>
// where PIN: PinOps<Dynamic = port::Dynamic>
// {
//     pub fn into_pin(self) -> Pin<mode::Input<mode::PullUp>, PIN>
//     {
//         let reader_system = unsafe { &mut _READER_SYSTEM }.as_mut().unwrap();

//         interrupt::free(|_|
//         {
//             reader_system._readers[self._reader_index as usize] = None;
//         });

//         unsafe { self._ghost_pin.assume_init() }
//     }
// }

// pub fn init_soft_serial() -> Result<(), ()>
// {
//     unsafe
//     {
//         if let None = _READER_SYSTEM
//         {
//             // https://www.arxterra.com/11-atmega328p-external-interrupts/#External_Interrupts

//             interrupt::disable();

//             let dp =  Peripherals::steal();

//             dp.EXINT.pcicr.write(|w| w
//                 .pcie().bits(0b111));
//             dp.EXINT.eicra.write(|w| w
//                 .isc0().bits(0b01));

//             dp.TC1.timsk1.write(|w| w
//                 .ocie1a().clear_bit());
//             dp.TC1.tccr1a.write(|w| w
//                 .wgm1().bits(0b__00));
//             dp.TC1.tccr1b.write(|w| w
//                 .wgm1().bits(0b01__)
//                 .cs1().bits(_PRESCALER.cs_bits()));

//             _READER_SYSTEM = Some(_SoftSerialReaderHandler::new(dp));

//             interrupt::enable();

//             Ok(())
//         }
//         else { Err(()) }
//     }
// }

// #[derive(Debug, Clone, Copy)]
// enum _SoftSerialReaderStateProgress
// {
//     Waiting,
//     StartBit,
//     Bit0,
//     Bit1,
//     Bit2,
//     Bit3,
//     Bit4,
//     Bit5,
//     Bit6,
//     Bit7,
//     EndBit,
// }

// struct _SoftSerialReaderState
// {
//     _pin: Pin<mode::Input<mode::PullUp>, port::Dynamic>,
//     _pin_was_high: bool,
//     _callback: &'static dyn FnMut(u8, CriticalSection),
//     _byte_accumulate: u8,
//     _progress: _SoftSerialReaderStateProgress,
//     _cycles_between_last_ovf_and_next_compa: u32,
//     _cycles_per_bit: u32,
//     _cycles_per_one_and_a_half_bits: u32,
//     _high_equals_one: bool,
// }

// struct _SoftSerialReaderStateContext<FnA: FnOnce(u32)>
// {
//     pub tcnt: u16,
//     pub schedule_compa: FnA,
// }

// impl _SoftSerialReaderState
// {
//     pub fn on_pcint(
//         &mut self,
//         context: _SoftSerialReaderStateContext<impl FnOnce(u32)>,
//         _: _PCINTGroup)
//     {
//         match self._progress
//         {
//             _SoftSerialReaderStateProgress::Waiting =>
//             {
//                 if self._pin_was_high == self._high_equals_one
//                 {
//                     (context.schedule_compa)(
//                         self._cycles_per_one_and_a_half_bits);

//                     self._progress = _SoftSerialReaderStateProgress::StartBit;
//                 }
//             },
//             _ => (),
//         }
//     }

//     pub fn on_timer1_compa(
//         &mut self,
//         context: _SoftSerialReaderStateContext<impl FnOnce(u32)>)
//     {
//         match self._progress
//         {
//             _SoftSerialReaderStateProgress::StartBit =>
//             {
//                 if self._pin.is_high() == self._high_equals_one
//                 {
//                     (context.schedule_compa)(
//                         self._cycles_per_bit);

//                     self._progress = _SoftSerialReaderStateProgress::StartBit;
//                 }
//             },
//             _ => (),
//         }
//     }
// }

// struct _SoftSerialReaderHandler
// {
//     _dp: Peripherals,
//     _readers: [Option<_SoftSerialReaderState>; MAX_SERIAL_READERS],
//     _next_compa_target: Option<u8>,
// }

// impl _SoftSerialReaderHandler
// {
//     pub fn new(dp: Peripherals) -> Self
//     {
//         _SoftSerialReaderHandler
//         {
//             _dp: dp,
//             _readers: array::from_fn(|_| None),
//             _next_compa_target: None,
//         }
//     }

//     pub fn on_pcint(&mut self, group: _PCINTGroup)
//     {
//         let tcnt = self._dp.TC1.tcnt1.read().bits();

//         for reader in &mut self._readers
//         {
//             let Some(reader) = reader else { continue };

//             let pin_is_high = reader._pin.is_high();

//             if pin_is_high != reader._pin_was_high
//             {
//                 reader._pin_was_high = pin_is_high;

//                 reader.on_pcint(
//                     _SoftSerialReaderStateContext
//                     {
//                         tcnt,
//                         schedule_compa: |cycles|
//                         {
//                             let offset_tcnt = tcnt.wrapping_add(cycles as u16);
//                             cycles
//                         },
//                     },
//                     group);
//             }
//         }
//     }

//     pub fn on_timer1_ovf(&mut self)
//     {
//         for reader in &mut self._readers
//         {
//             let Some(reader) = reader else { continue };

//             reader.on_timer1_ovf(
//                 ||
//                 {

//                 });
//         }
//     }

//     pub fn on_timer1_compa(&mut self)
//     {
//         for reader in &mut self._readers
//         {
//             let Some(reader) = reader else { continue };

//             reader.on_timer1_compa();
//         }
//     }
// }

// static mut _READER_SYSTEM: Option<_SoftSerialReaderHandler> = None;

// #[derive(Debug, Clone, Copy)]
// #[allow(unused)]
// enum _Prescaler
// {
//     None,
//     By8,
//     By64,
//     By256,
//     By1024,
// }

// impl _Prescaler
// {
//     pub const fn cs_bits(&self) -> u8
//     {
//         match self
//         {
//             _Prescaler::None => 0b001,
//             _Prescaler::By8 => 0b010,
//             _Prescaler::By64 => 0b011,
//             _Prescaler::By256 => 0b100,
//             _Prescaler::By1024 => 0b101,
//         }
//     }

//     pub const fn value(&self) -> u16
//     {
//         match self
//         {
//             _Prescaler::None => 1,
//             _Prescaler::By8 => 8,
//             _Prescaler::By64 => 64,
//             _Prescaler::By256 => 256,
//             _Prescaler::By1024 => 1024,
//         }
//     }
// }

// #[derive(Debug, Clone, Copy)]
// enum _PCINTGroup
// {
//     PCINT0,
//     PCINT1,
//     PCINT2,
// }

// // MARK: Interrupts
// #[interrupt(atmega328p)]
// unsafe fn PCINT0() { _pcint(_PCINTGroup::PCINT0) }

// #[interrupt(atmega328p)]
// unsafe fn PCINT1() { _pcint(_PCINTGroup::PCINT1) }

// #[interrupt(atmega328p)]
// unsafe fn PCINT2() { _pcint(_PCINTGroup::PCINT2) }

// unsafe fn _pcint(group: _PCINTGroup)
// {
//     _READER_SYSTEM.as_mut().map(|reader| reader.on_pcint(group));
// }

// #[interrupt(atmega328p)]
// unsafe fn TIMER1_OVF()
// {
//     _READER_SYSTEM.as_mut().map(|reader| reader.on_timer1_ovf());
// }

// #[interrupt(atmega328p)]
// unsafe fn TIMER1_COMPA()
// {
//     _READER_SYSTEM.as_mut().map(|reader| reader.on_timer1_compa());
// }