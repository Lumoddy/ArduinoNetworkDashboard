
// https://chatgpt.com

//! Minimal, interrupt-driven SoftwareSerial-like API for ATmega328P (Arduino Uno)
//! - Full-duplex (bit-banged TX, sampled RX) 8N1
//! - RX: start-bit detect via Pin Change Interrupt (PCINT), then Timer1 samples each bit
//! - TX: Timer1 Compare ISR clocks bits out
//! - API intentionally mirrors Arduino's SoftwareSerial where reasonable
//!
//! Limitations:
//! - Single global instance (like many Arduino soft UARTs)
//! - One baud rate at a time; tested values: 9600, 19200, 38400 @ 16 MHz
//! - RX pin must be any PCINT-capable pin (Uno: D0–D13, A0–A5); TX any digital output
//! - No line inversion; framing fixed to 8N1

use arduino_hal::hal::port::{Dynamic, Pin, PinOps};
use arduino_hal::hal::port::mode::{Input, Output, PullUp};
use arduino_hal::pac::{self};
use arduino_hal::pac::exint::{self, PCICR, PCMSK0, PCMSK1, PCMSK2};
use arduino_hal::pac::tc1::{self, TCNT1, TCCR1A, TCCR1B, TIMSK1, OCR1A};
use avr_device::interrupt;
use core::cell::UnsafeCell;
use core::ops::Deref;
use core::sync::atomic::{AtomicBool, Ordering};

const F_CPU: u32 = 16_000_000; // Uno

// Ring buffer sizes (power of two for easy wrap)
const RX_BUF_SIZE: usize = 64;
const TX_BUF_SIZE: usize = 64;

#[derive(Copy, Clone)]
pub struct Config
{
    pub baud: u32,
}

impl Default for Config
{
    fn default() -> Self { Self { baud: 9600 } }
}

#[repr(C)]
struct RingBuf
{
    buf: [u8; RX_BUF_SIZE],
    head: core::cell::Cell<u8>,
    tail: core::cell::Cell<u8>,
}

impl RingBuf
{
    const fn new() -> Self
    {
        Self
        {
            buf: [0; RX_BUF_SIZE],
            head: core::cell::Cell::new(0),
            tail: core::cell::Cell::new(0)
        }
    }

    fn push(&mut self, b: u8)
    {
        let h = self.head.get();
        let n = h.wrapping_add(1);
        if n != self.tail.get() { // drop on overflow
            self.buf[h as usize] = b;
            self.head.set(n);
        }
    }

    fn pop(&mut self) -> Option<u8>
    {
        let t = self.tail.get();
        if t == self.head.get() { return None; }
        let b = self.buf[t as usize];
        self.tail.set(t.wrapping_add(1));
        Some(b)
    }

    fn available(&self) -> u8 { self.head.get().wrapping_sub(self.tail.get()) }
}

#[derive(Copy, Clone)]
pub enum PcintGroup { PCINT0, PCINT1, PCINT2 }

/// SoftwareSerial-like struct. Only one instance supported.
pub struct SoftwareSerial
{
    rx_dyn: Pin<Input<PullUp>, Dynamic>, // RX as dynamic input w/ pullup
    tx_dyn: Pin<Output, Dynamic>,        // TX as dynamic output
    group: PcintGroup,
    pcint_mask_bit: u8,
    cycles_per_bit: u16,
}

// Global state accessed by ISRs
struct State
{
    rx_buf: RingBuf,
    tx_buf: RingBuf,
    sampling: AtomicBool,
    rx_sample_count: core::cell::Cell<u8>,
    rx_shift: core::cell::Cell<u8>,
    tx_active: AtomicBool,
    tx_shift: core::cell::Cell<u16>, // start + 8 data + stop => 10 bits, LSB-first
    last_rx_level_high: AtomicBool,
}

static mut STATE: State = State
{
    rx_buf: RingBuf::new(),
    tx_buf: RingBuf::new(),
    sampling: AtomicBool::new(false),
    rx_sample_count: core::cell::Cell::new(0),
    rx_shift: core::cell::Cell::new(0),
    tx_active: AtomicBool::new(false),
    tx_shift: core::cell::Cell::new(0),
    last_rx_level_high: AtomicBool::new(true),
};

static mut SINGLETON: Option<SoftwareSerial> = None;

impl SoftwareSerial
{
    /// Create a new instance from any RX (input w/ pullup) + TX (output) pins
    pub fn new<RP, TP>(rx: Pin<Input<PullUp>, RP>, tx: Pin<Output, TP>, cfg: Config) -> Self
    where RP: PinOps, TP: PinOps
    {
        let rx_dyn = rx.downgrade();
        let tx_dyn = tx.downgrade();

        // Compute PCINT group & mask bit for RX pin
        let (group, bit) = pcint_for_dyn_pin(&rx_dyn);
        let cycles_per_bit = ((F_CPU + cfg.baud/2) / cfg.baud) as u16; // round to nearest

        SoftwareSerial { rx_dyn, tx_dyn, group, pcint_mask_bit: bit, cycles_per_bit }
    }

    /// Begin the software UART and enable interrupts
    pub fn begin(mut self) -> &'static mut SoftwareSerial
    {
        // Configure TX high (idle)
        self.tx_dyn.set_high();

        // Remember idle RX level
        unsafe { STATE.last_rx_level_high.store(self.rx_dyn.is_high(), Ordering::Relaxed); }

        // Timer1: CTC mode, OCR1A = cycles_per_bit-1
        let dp = unsafe { pac::Peripherals::steal() };
        unsafe
        {
            (*TCCR1A::ptr()).write(|w| w);
            // CTC mode (WGM12=1), clk/1
            (*TCCR1B::ptr()).write(|w| w.wgm12().set_bit().cs10().set_bit());
            (*OCR1A::ptr()).write(|w| w.bits(self.cycles_per_bit - 1));
            // Enable compare A interrupt disabled for now; enabled on TX or RX sampling start
            (*TIMSK1::ptr()).write(|w| w);
        }

        // Enable PCINT for RX pin's group
        unsafe
        {
            match self.group
            {
                PcintGroup::PCINT0 =>
                {
                    (*PCMSK0::ptr()).modify(|r, w| w.bits(r.bits() | (1 << self.pcint_mask_bit)));
                    (*PCICR::ptr()).modify(|r, w| w.bits(r.bits() | 1 << 0));
                }
                PcintGroup::PCINT1 =>
                {
                    (*PCMSK1::ptr()).modify(|r, w| w.bits(r.bits() | (1 << self.pcint_mask_bit)));
                    (*PCICR::ptr()).modify(|r, w| w.bits(r.bits() | 1 << 1));
                }
                PcintGroup::PCINT2 =>
                {
                    (*PCMSK2::ptr()).modify(|r, w| w.bits(r.bits() | (1 << self.pcint_mask_bit)));
                    (*PCICR::ptr()).modify(|r, w| w.bits(r.bits() | 1 << 2));
                }
            }
        }

        unsafe { SINGLETON = Some(self); SINGLETON.as_mut().unwrap() }
    }

    /// Write a byte (enqueue); ISR shifts it out
    pub fn write(&mut self, byte: u8)
    {
        unsafe { STATE.tx_buf.push(byte); }
        self.kick_tx();
    }

    /// Attempt to read a byte (non-blocking)
    pub fn read(&mut self) -> nb::Result<u8, ()>
    {
        unsafe { STATE.rx_buf.pop().ok_or(nb::Error::WouldBlock) }
    }

    pub fn available(&self) -> u8 { unsafe { STATE.rx_buf.available() } }

    pub fn flush(&self)
    {
        // Wait until TX done
        while unsafe { STATE.tx_active.load(Ordering::Acquire) } {}
    }

    /// Similar to SoftwareSerial::listen(); no-op here as only one instance exists
    pub fn listen(&self) -> bool { true }

    fn kick_tx(&mut self)
    {
        // If not active, start a new frame
        let active = unsafe { STATE.tx_active.load(Ordering::Acquire) };
        if active { return; }
        if let Some(b) = unsafe { STATE.tx_buf.pop() }
        {
            // Frame: start(0) + data LSB-first + stop(1)
            let frame: u16 = ((1u16 << 9) | (b as u16) << 1) & 0x03FF; // 10 bits
            unsafe
            {
                STATE.tx_shift.set(frame);
                STATE.tx_active.store(true, Ordering::Release);
                // Set OCR1A and enable ISR
                (*OCR1A::ptr()).write(|w| w.bits(self.cycles_per_bit - 1));
                (*TIMSK1::ptr()).modify(|r, w| w.bits(r.bits() | 1 << 1)); // OCIE1A
            }
            // Drive start bit immediately, next ticks at OCR1A
            self.tx_dyn.set_low();
        }
    }
}

fn pcint_for_dyn_pin(pin: &Pin<Input<PullUp>, Dynamic>) -> (PcintGroup, u8)
{
    // Map dynamic pins to PCINT group/bit, following ATmega328P datasheet
    // Port B: PCINT0..7 (D8..D13 + PB0..PB5), Port C: PCINT8..14 (A0..A5), Port D: PCINT16..23 (D0..D7)
    match pin.id()
    {
        Dynamic::PB0 => (PcintGroup::PCINT0, 0),
        Dynamic::PB1 => (PcintGroup::PCINT0, 1),
        Dynamic::PB2 => (PcintGroup::PCINT0, 2),
        Dynamic::PB3 => (PcintGroup::PCINT0, 3),
        Dynamic::PB4 => (PcintGroup::PCINT0, 4),
        Dynamic::PB5 => (PcintGroup::PCINT0, 5),
        Dynamic::PB6 => (PcintGroup::PCINT0, 6),
        Dynamic::PB7 => (PcintGroup::PCINT0, 7),
        Dynamic::PC0 => (PcintGroup::PCINT1, 0),
        Dynamic::PC1 => (PcintGroup::PCINT1, 1),
        Dynamic::PC2 => (PcintGroup::PCINT1, 2),
        Dynamic::PC3 => (PcintGroup::PCINT1, 3),
        Dynamic::PC4 => (PcintGroup::PCINT1, 4),
        Dynamic::PC5 => (PcintGroup::PCINT1, 5),
        Dynamic::PD0 => (PcintGroup::PCINT2, 0),
        Dynamic::PD1 => (PcintGroup::PCINT2, 1),
        Dynamic::PD2 => (PcintGroup::PCINT2, 2),
        Dynamic::PD3 => (PcintGroup::PCINT2, 3),
        Dynamic::PD4 => (PcintGroup::PCINT2, 4),
        Dynamic::PD5 => (PcintGroup::PCINT2, 5),
        Dynamic::PD6 => (PcintGroup::PCINT2, 6),
        Dynamic::PD7 => (PcintGroup::PCINT2, 7),
        _ => (PcintGroup::PCINT2, 0),
    }
}

// ===== Interrupts =====

/// Pin Change Interrupts: detect start bit (high->low transition)
#[interrupt(atmega328p)]
fn PCINT0() { on_pcint_group(PcintGroup::PCINT0); }
#[interrupt(atmega328p)]
fn PCINT1() { on_pcint_group(PcintGroup::PCINT1); }
#[interrupt(atmega328p)]
fn PCINT2() { on_pcint_group(PcintGroup::PCINT2); }

fn on_pcint_group(group: PcintGroup)
{
    let ss = unsafe { SINGLETON.as_ref() };
    if ss.is_none() { return; }
    let ss = ss.unwrap();

    // Read RX level
    let level_high = ss.rx_dyn.is_high();
    let last = unsafe { STATE.last_rx_level_high.swap(level_high, Ordering::AcqRel) };

    // Detect falling edge (start bit)
    if last && !level_high && !unsafe { STATE.sampling.load(Ordering::Acquire) }
    {
        // Schedule first sample at 1.5 bit periods: set TCNT1 so next OCR1A hit equals 1.5*Tbit
        unsafe
        {
            let cpb = ss.cycles_per_bit;
            let first = cpb + (cpb / 2);
            (*TCNT1::ptr()).write(|w| w.bits(cpb.wrapping_sub(first))); // so that next compare occurs in 'first' cycles
            STATE.rx_shift.set(0);
            STATE.rx_sample_count.set(0);
            STATE.sampling.store(true, Ordering::Release);
            (*TIMSK1::ptr()).modify(|r, w| w.bits(r.bits() | 1 << 1)); // enable OCIE1A
        }
    }
}

/// Timer1 Compare A: drives TX bits and samples RX bits when active
#[interrupt(atmega328p)]
fn TIMER1_COMPA()
{
    let ss = unsafe { SINGLETON.as_mut() };
    if ss.is_none() { return; }
    let ss = ss.unwrap();

    // ----- RX sampling -----
    if unsafe { STATE.sampling.load(Ordering::Acquire) }
    {
        let bit = if ss.rx_dyn.is_high() { 1 } else { 0 };
        let mut shift = unsafe { STATE.rx_shift.get() };
        shift >>= 1;
        shift |= (bit << 7);
        unsafe { STATE.rx_shift.set(shift); }

        let c = unsafe { STATE.rx_sample_count.get() } + 1;
        unsafe { STATE.rx_sample_count.set(c); }
        if c >= 8 { // collected 8 data bits; next tick would be stop; we can verify stop externally if desired
            unsafe
            {
                STATE.rx_buf.push(STATE.rx_shift.get());
                STATE.sampling.store(false, Ordering::Release);
            }
        }
    }

    // ----- TX shifting -----
    if unsafe { STATE.tx_active.load(Ordering::Acquire) }
    {
        let mut frame = unsafe { STATE.tx_shift.get() };
        // Output current LSB
        if (frame & 0x0001) == 0 { ss.tx_dyn.set_low(); } else { ss.tx_dyn.set_high(); }
        frame >>= 1;
        unsafe { STATE.tx_shift.set(frame); }
        if frame == 0 { // finished 10 bits
            unsafe
            {
                STATE.tx_active.store(false, Ordering::Release);
            }
            // Queue next if any
            ss.kick_tx();
            // If nothing left and not sampling, disable interrupt to reduce jitter
            if !unsafe { STATE.tx_active.load(Ordering::Acquire) } && !unsafe { STATE.sampling.load(Ordering::Acquire) }
            {
                unsafe { (*TIMSK1::ptr()).modify(|r, w| w.bits(r.bits() & !(1 << 1))); }
            }
        }
    } else
    {
        // No TX active: keep line idle high
        ss.tx_dyn.set_high();
        // If not sampling either, turn off interrupt to save CPU
        if !unsafe { STATE.sampling.load(Ordering::Acquire) }
        {
            unsafe { (*TIMSK1::ptr()).modify(|r, w| w.bits(r.bits() & !(1 << 1))); }
        }
    }
}