use arduino_hal::{delay_ms, pac::wdt::wdtcsr};
use avr_device::interrupt;

use crate::format;

#[macro_export]
macro_rules! panic_payload
{
    ($($type:ident$( as $generic:ident)?: $value:expr),*) =>
    {
        {
            #[allow(unused_unsafe)]
            unsafe
            {
                $crate::interrupt::disable();
                $($crate::panic_setup_payload!($type$( as $generic)?: $value);)*
                panic!();
            }
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
macro_rules! unreachable_payload
{
    () =>
    {
        $crate::panic_payload!(
            str: b"Encountered unreachable at ",
            str: file!().as_bytes(),
            str: b"(",
            b10 as i32: line!(),
            str: b":",
            b10 as i32: column!(),
            str: b")")
    };
    ($($type:ident$( as $generic:ident)?: $value:expr),*) =>
    {
        $crate::panic_payload!(
            b"Encountered unreachable: ",
            $($type$( as $generic)?: $value),*);
    };
}

#[macro_export]
macro_rules! panic_setup_payload
{
    () => { { } };
    (bCustom as u8: ($value:expr, [$($letters:expr),*], $negative:expr$(,)?)) =>
    {
        _ = $crate::panic_handler::push_u8_to_payload(
            $value as u8,
            &[$($letters),*])
    };
    (bCustom as i8: ($value:expr, [$($letters:expr),*], $negative:expr$(,)?)) =>
    {
        _ = $crate::panic_handler::push_i8_to_payload(
            $value as i8,
            &[$($letters),*],
            $negative)
    };
    (bCustom as u16: ($value:expr, [$($letters:expr),*], $negative:expr$(,)?)) =>
    {
        _ = $crate::panic_handler::push_u16_to_payload(
            $value as u16,
            &[$($letters),*])
    };
    (bCustom as i16: ($value:expr, [$($letters:expr),*], $negative:expr$(,)?)) =>
    {
        _ = $crate::panic_handler::push_i16_to_payload(
            $value as i16,
            &[$($letters),*],
            $negative)
    };
    (bCustom as u32: ($value:expr, [$($letters:expr),*], $negative:expr$(,)?)) =>
    {
        _ = $crate::panic_handler::push_u32_to_payload(
            $value as u32,
            &[$($letters),*])
    };
    (bCustom as i32: ($value:expr, [$($letters:expr),*], $negative:expr$(,)?)) =>
    {
        _ = $crate::panic_handler::push_i32_to_payload(
            $value as i32,
            &[$($letters),*],
            $negative)
    };
    (bCustom as u64: ($value:expr, [$($letters:expr),*], $negative:expr$(,)?)) =>
    {
        _ = $crate::panic_handler::push_u64_to_payload(
            $value as u64,
            &[$($letters),*])
    };
    (bCustom as i64: ($value:expr, [$($letters:expr),*], $negative:expr$(,)?)) =>
    {
        _ = $crate::panic_handler::push_i64_to_payload(
            $value as i64,
            &[$($letters),*],
            $negative)
    };
    (bCustom as usize: ($value:expr, [$($letters:expr),*], $negative:expr$(,)?)) =>
    {
        _ = $crate::panic_handler::push_usize_to_payload(
            $value as usize,
            &[$($letters),*])
    };
    (bCustom as isize: ($value:expr, [$($letters:expr),*], $negative:expr$(,)?)) =>
    {
        _ = $crate::panic_handler::push_isize_to_payload(
            $value as isize,
            &[$($letters),*],
            $negative)
    };
    (b10 as $generic:ident: $value:expr) =>
    {
        $crate::panic_setup_payload!(bCustom as $generic:
            (
                $value,
                [b"0", b"1", b"2", b"3", b"4", b"5", b"6", b"7", b"8", b"9"],
                b"-",
            ));
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
    ($vis:vis unsafe fn $name:ident $num:ty as signed $base:ident;) =>
    {
        $vis unsafe fn $name(
            value: $num,
            alphabet: &[&'static [u8]],
            negative: &'static [u8])
        {
            $crate::format::$base(
                value,
                alphabet,
                negative,
                |char| unsafe { push_str_to_payload(char) });
        }
    };
    ($vis:vis unsafe fn $name:ident $num:ty as unsigned $base:ident;) =>
    {
        $vis unsafe fn $name(
            value: $num,
            alphabet: &[&'static [u8]])
        {
            $crate::format::$base(
                value,
                alphabet,
                |char| unsafe { push_str_to_payload(char) });
        }
    };
    (
        $first_vis:vis unsafe fn $first_name:ident $first_num:ty as $first_sign:ident $first_base:ident;
        $($rest_vis:vis unsafe fn $rest_name:ident $rest_num:ty as $rest_sign:ident $rest_base:ident;)+
    ) =>
    {
        push_x_to_payload_impl!
        {
            $first_vis unsafe fn $first_name $first_num as $first_sign $first_base;
        }
        $(
            push_x_to_payload_impl!
            {
                $rest_vis unsafe fn $rest_name $rest_num as $rest_sign $rest_base;
            }
        )+
    };
}

pub unsafe fn push_str_to_payload(value: &'static [u8])
{
    _ = _PAYLOAD.push(value);
}

push_x_to_payload_impl!
{
    pub unsafe fn push_u8_to_payload u8 as unsigned push_u8;
    pub unsafe fn push_i8_to_payload i8 as signed push_i8;
    pub unsafe fn push_u16_to_payload u16 as unsigned push_u16;
    pub unsafe fn push_i16_to_payload i16 as signed push_i16;
    pub unsafe fn push_u32_to_payload u32 as unsigned push_u32;
    pub unsafe fn push_i32_to_payload i32 as signed push_i32;
    pub unsafe fn push_u64_to_payload u64 as unsigned push_u64;
    pub unsafe fn push_i64_to_payload i64 as signed push_i64;
    pub unsafe fn push_usize_to_payload usize as unsigned push_usize;
    pub unsafe fn push_isize_to_payload isize as signed push_isize;
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

    for byte in b"!Resetting...\n"
    { serial.write_byte(*byte) }

    delay_ms(1000);

    dp.CPU.mcusr.write(|w| w
        .wdrf().set_bit());
    dp.WDT.wdtcsr.write(|w| w
        .wde().set_bit()
        .wdph().variant(false)
        .wdpl().variant(wdtcsr::WDPL_A::CYCLES_64K));

    loop { }
}