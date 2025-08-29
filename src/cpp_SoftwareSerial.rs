// Direct copy and translation of C++ code from
// https://github.com/arduino/ArduinoCore-avr/blob/master/libraries/SoftwareSerial/src/SoftwareSerial.h
// https://github.com/arduino/ArduinoCore-avr/blob/master/libraries/SoftwareSerial/src/SoftwareSerial.cpp

const _SS_MAX_RX_BUFF: usize = 64;

pub struct SoftwareSerial
{
    _receive_pin: u8,
    _receive_bit_mask_l: u8,
    _receive_port_register: *mut u8,
    _transmit_bit_mask: u8,
    _transmit_port_register: *mut u8,
    _pcint_maskreg: *mut u8,
    _pcint_maskvalue: u8,

    /// Expressed as 4-cycle delays (must never be 0!)
    _rx_delay_centering: u16,
    _rx_delay_intrabit: u16,
    _rx_delay_stopbit: u16,
    _tx_delay: u16,

    _buffer_overflow: bool,
    _inverse_logic: bool,
}

// static data
const _DEBUG: bool = false;
static mut _RECEIVE_BUFFER: [u8; _SS_MAX_RX_BUFF] = [0; _SS_MAX_RX_BUFF];
static mut _RECEIVE_BUFFER_TAIL: u8 = 0;
static mut _RECEIVE_BUFFER_HEAD: u8 = 0;
static mut _ACTIVE_OBJECT: Option<&mut SoftwareSerial> = None;

pub fn debug_pulse(pin: u8, count: u8)
{
    if !_DEBUG { return };

    unsafe
    {
        let port = port_output_register(digital_pin_to_port(pin));

        let val = core::ptr::read_volatile(port);
        loop
        {
            count -= 1;
            if count == 0 { break };

            core::ptr::write_volatile(port, val | digital_pin_to_bit_mask(pin));
            core::ptr::write_volatile(port, val);
        }
    }
}

impl SoftwareSerial
{
    // private methods
    fn _recv(&mut self)
    {

    }

    fn _rx_pin_read(&mut self) -> u8
    {

    }

    fn _set_tx(&mut self, transmit_pin: u8)
    {

    }

    fn _set_rx(&mut self, receive_pin: u8)
    {

    }

    fn _set_rx_int_msk(&mut self, enable: bool)
    {

    }

    /// Return num - sub, or 1 if the result would be < 1
    fn _subtract_cap(num: u16, sub: u16) -> u16
    {

    }

    /// private static method for timing
    fn _tuned_delay(delay: u16)
    {
        _delay_loop_2(delay);
    }
}

impl Drop for SoftwareSerial
{
    fn drop(&mut self)
    {
        
    }
}

impl SoftwareSerial
{
    // public methods
    pub fn new(receive_pin: u8, transmit_pin: u8, inverse_logic: bool)
    {

    }

    pub fn begin(&mut self, speed: i32)
    {

    }

    pub fn listen(&mut self) -> Result<(), ()>
    {
        if self._rx_delay_stopbit == 0 { return Err(()) };

        if
        {
            if let Some(active_object) = unsafe { &_ACTIVE_OBJECT }
            {
                *active_object as *const SoftwareSerial
                    != self as *const SoftwareSerial
            }
            else
            { false }
        }
        {
            if let Some(active_object) = unsafe { &mut _ACTIVE_OBJECT }
            { active_object.stop_listening() }

            self._buffer_overflow = false;
            unsafe
            {
                _RECEIVE_BUFFER_HEAD = 0;
                _RECEIVE_BUFFER_TAIL = 0;
                _ACTIVE_OBJECT = Some(self);
            }
        }

        return Err(());
    }

    pub fn end(&mut self)
    {

    }

    pub fn is_listening(&self) -> bool
    {
        if let Some(pointer) = unsafe { &_ACTIVE_OBJECT }
        {
            self as *const SoftwareSerial
                == *pointer as *const SoftwareSerial
        }
        else { false }
    }

    pub fn stop_listening(&mut self)
    {

    }

    pub fn overflow(&mut self) -> bool
    {
        let ret = self._buffer_overflow;
        self._buffer_overflow = false;
        return ret;
    }

    pub fn peek(&mut self) -> Result<u8, ()>
    {

    }

    pub fn write(&mut self, byte: u8) -> usize
    {

    }

    pub fn read(&mut self) -> Option<u8>
    {

    }

    pub fn available(&self) -> usize
    {

    }

    pub fn flush(&mut self)
    {

    }

    /// public only for easy access by interrupt handlers.
    pub fn _handle_interrupt()
    {

    }
}