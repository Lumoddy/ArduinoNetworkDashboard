use arduino_hal::{delay_ms, pac::wdt::wdtcsr};
use avr_device::interrupt;

#[macro_export]
macro_rules! panic_payload
{
    ($($type:ident$( as $generic:ident)?: $value:expr),*) =>
    {
        #[allow(unused_unsafe)]
        unsafe
        {
            $crate::interrupt::disable();
            $($crate::panic_setup_payload!($type$( as $generic)?: $value);)*
            panic!();
        }
    };
}

#[macro_export]
macro_rules! unwrap_payload
{
    ($result:expr) =>
    {
        $result.or_else(
            |_| -> Result<_, Infallible>
            {
                $crate::panic_payload!(
                    str: b"Failed unwrap at ",
                    str: file!().as_bytes(),
                    str: b"(",
                    b10 as i32: line!(),
                    str: b":",
                    b10 as i32: column!(),
                    str: b")")
            })
            .unwrap_infallible()
    };
    ($result:expr, $($type:ident$( as $generic:ident)?: $value:expr),*) =>
    {
        $result.or_else(
            |_| -> Result<_, Infallible>
            {
                $crate::panic_payload!(
                    b"Failed unwrap: ",
                    $($type$( as $generic)?: $value),*);
            })
            .unwrap_infallible()
    };
}

#[macro_export]
macro_rules! expect_payload
{
    ($($type:ident$( as $generic:ident)?: $value:expr),*) =>
    {
        $result.or_else(
            |_| -> Result<_, Infallible>
            {
                $crate::panic_payload!($($type$( as $generic)?: $value),*);
            })
            .unwrap_infallible()
    };
}

#[macro_export]
macro_rules! panic_setup_payload
{
    () => { { } };
    (bCustom as u8: ($value:expr, [$($letters:expr),*])) =>
    {
        _ = $crate::panic_handler::push_u8_to_payload($value as u8, &[$($letters),*])
    };
    (bCustom as i8: ($value:expr, [$($letters:expr),*])) =>
    {
        _ = $crate::panic_handler::push_i8_to_payload($value as i8, &[$($letters),*])
    };
    (bCustom as u16: ($value:expr, [$($letters:expr),*])) =>
    {
        _ = $crate::panic_handler::push_u16_to_payload($value as u16, &[$($letters),*])
    };
    (bCustom as i16: ($value:expr, [$($letters:expr),*])) =>
    {
        _ = $crate::panic_handler::push_i16_to_payload($value as i16, &[$($letters),*])
    };
    (bCustom as u32: ($value:expr, [$($letters:expr),*])) =>
    {
        _ = $crate::panic_handler::push_u32_to_payload($value as u32, &[$($letters),*])
    };
    (bCustom as i32: ($value:expr, [$($letters:expr),*])) =>
    {
        _ = $crate::panic_handler::push_i32_to_payload($value as i32, &[$($letters),*])
    };
    (bCustom as u64: ($value:expr, [$($letters:expr),*])) =>
    {
        _ = $crate::panic_handler::push_u64_to_payload($value as u64, &[$($letters),*])
    };
    (bCustom as i64: ($value:expr, [$($letters:expr),*])) =>
    {
        _ = $crate::panic_handler::push_i64_to_payload($value as i64, &[$($letters),*])
    };
    (bCustom as usize: ($value:expr, [$($letters:expr),*])) =>
    {
        _ = $crate::panic_handler::push_usize_to_payload($value as usize, &[$($letters),*])
    };
    (bCustom as isize: ($value:expr, [$($letters:expr),*])) =>
    {
        _ = $crate::panic_handler::push_isize_to_payload($value as isize, &[$($letters),*])
    };
    (b10 as $generic:ident: $value:expr) =>
    {
        $crate::panic_setup_payload!(bCustom as $generic:
            ($value, [b"0", b"1", b"2", b"3", b"4", b"5", b"6", b"7", b"8", b"9"]));
    };
    (bool: $value:expr) =>
    {
        _ = $crate::panic_handler::push_str_to_payload(
            if $value as bool { b"true" } else { b"false" })
    };
    (str: $value:expr) =>
    {
        _ = $crate::panic_handler::push_str_to_payload($value)
    };
}

#[allow(unused)]
static mut _PAYLOAD: heapless::Vec<&'static [u8], 32> = heapless::Vec::new();

macro_rules! push_x_to_payload_impl
{
    ($vis:vis unsafe fn $name:ident(value: $type:ty, signed: false)$(;)?) =>
    {
        $vis unsafe fn $name(value: $type, alphabet: &[&'static [u8]])
        {
            let mut value = value;
            let insert_index = _PAYLOAD.len();

            loop
            {
                let digit = value % alphabet.len() as $type;
                value /= alphabet.len() as $type;

                _ = _PAYLOAD.insert(insert_index, alphabet[digit as usize]);

                if value == 0 { break };
            }
        }
    };
    ($vis:vis unsafe fn $name:ident(value: $type:ty, signed: true)$(;)?) =>
    {
        $vis unsafe fn $name(value: $type, alphabet: &[&'static [u8]])
        {
            let mut value = value;
            let insert_index = _PAYLOAD.len();

            if value < 0
            {
                _ = _PAYLOAD.push(b"-");
                value = -value;
            }

            loop
            {
                let digit = value % alphabet.len() as $type;
                value /= alphabet.len() as $type;

                _ = _PAYLOAD.insert(insert_index, alphabet[digit as usize]);

                if value == 0 { break };
            }
        }
    };
    (
        $first_vis:vis unsafe fn $first_name:ident(
            value: $first_type:ty,
            signed: $first_signed:tt)$(;)?
        $($rest_vis:vis unsafe fn $rest_name:ident(
            value: $rest_type:ty,
            signed: $rest_signed:tt)$(;)?)+
    ) =>
    {
        push_x_to_payload_impl!($first_vis unsafe fn $first_name(
            value: $first_type,
            signed: $first_signed));
        $(push_x_to_payload_impl!($rest_vis unsafe fn $rest_name(
            value: $rest_type,
            signed: $rest_signed));)+
    };
}

pub unsafe fn push_str_to_payload(value: &'static [u8])
{
    _ = _PAYLOAD.push(value);
}

push_x_to_payload_impl!
{
    pub unsafe fn push_u8_to_payload(value: u8, signed: false);
    pub unsafe fn push_i8_to_payload(value: i8, signed: true);
    pub unsafe fn push_u16_to_payload(value: u16, signed: false);
    pub unsafe fn push_i16_to_payload(value: i16, signed: true);
    pub unsafe fn push_u32_to_payload(value: u32, signed: false);
    pub unsafe fn push_i32_to_payload(value: i32, signed: true);
    pub unsafe fn push_u64_to_payload(value: u64, signed: false);
    pub unsafe fn push_i64_to_payload(value: i64, signed: true);
    pub unsafe fn push_usize_to_payload(value: usize, signed: false);
    pub unsafe fn push_isize_to_payload(value: isize, signed: true);
}

pub unsafe fn push_int_to_payload<
    I: core::ops::Rem<I, Output: Into<usize>>
        + core::ops::DivAssign<I>
        + core::cmp::PartialEq<I>
        + core::marker::Copy
        + core::ops::Neg<Output: Into<I>>
        + core::cmp::PartialOrd<I>
        + From<usize>>(value: I, alphabet: &[&'static [u8]])
{
    let mut value = value;
    let insert_index = _PAYLOAD.len();

    if value < 0.into()
    {
        _ = _PAYLOAD.push(b"-");
        value = (-value).into();
    }

    loop
    {
        let digit = value % alphabet.len().into();
        value /= alphabet.len().into();

        _ = _PAYLOAD.insert(insert_index, alphabet[digit.into()]);

        if value == 0.into() { break };
    }
}

#[inline(never)]
#[panic_handler]
unsafe fn panic(_: &core::panic::PanicInfo) -> !
{
    interrupt::disable();

    delay_ms(200);

    let dp = arduino_hal::Peripherals::steal();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::Usart::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(),
        arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(9600));

    if _PAYLOAD.len() == 0
    {
        for byte in b"!Panic\n"
        { serial.write_byte(*byte) }
    }
    else
    {
        for byte in b"!Panic \""
        { serial.write_byte(*byte) }
        for byte in _PAYLOAD.iter().flat_map(|x| x.iter())
        {
            match *byte
            {
                b'"' => serial.write_byte(b'\\'),
                _ => (),
            }

            serial.write_byte(*byte);
        }
        for byte in b"\"\n"
        { serial.write_byte(*byte) }
    }

    for byte in b"Resetting...\n"
    { serial.write_byte(*byte) }

    delay_ms(1000);

    dp.WDT.wdtcsr.write(|w| w
        .wde().set_bit()
        .wdph().variant(false)
        .wdpl().variant(wdtcsr::WDPL_A::CYCLES_64K));

    loop { }
}