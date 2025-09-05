use arduino_hal::hal::port;

pub trait PCINTPin
{
    unsafe fn enable_int();

    unsafe fn disable_int();

    unsafe fn int_is_enabled() -> bool;
}

macro_rules! pcint_pin_impl
{
    (
        $(
            impl PCINTPin for $pin:ty
            {
                $pcmsk:ident & $mask:expr
            }
        )+
    ) =>
    {
        $(
            impl PCINTPin for $pin
            {
                unsafe fn enable_int()
                {
                    let dp = arduino_hal::Peripherals::steal();

                    dp.EXINT.$pcmsk.modify(|r, w| w
                        .pcint().bits(r.pcint().bits() | $mask));
                }

                unsafe fn disable_int()
                {
                    let dp = arduino_hal::Peripherals::steal();

                    dp.EXINT.$pcmsk.modify(|r, w| w
                        .pcint().bits(r.pcint().bits() & !($mask)));
                }

                unsafe fn int_is_enabled() -> bool
                {
                    let dp = arduino_hal::Peripherals::steal();

                    (dp.EXINT.$pcmsk.read().bits() & $mask) != 0
                }
            }
        )+
    };
}

pcint_pin_impl!
{
    impl PCINTPin for port::PB0 { pcmsk0 & (1 << 0) }
    impl PCINTPin for port::PB1 { pcmsk0 & (1 << 1) }
    impl PCINTPin for port::PB2 { pcmsk0 & (1 << 2) }
    impl PCINTPin for port::PB3 { pcmsk0 & (1 << 3) }
    impl PCINTPin for port::PB4 { pcmsk0 & (1 << 4) }
    impl PCINTPin for port::PB5 { pcmsk0 & (1 << 5) }
    impl PCINTPin for port::PB6 { pcmsk0 & (1 << 6) }
    impl PCINTPin for port::PB7 { pcmsk0 & (1 << 7) }

    impl PCINTPin for port::PC0 { pcmsk1 & (1 << 0) }
    impl PCINTPin for port::PC1 { pcmsk1 & (1 << 1) }
    impl PCINTPin for port::PC2 { pcmsk1 & (1 << 2) }
    impl PCINTPin for port::PC3 { pcmsk1 & (1 << 3) }
    impl PCINTPin for port::PC4 { pcmsk1 & (1 << 4) }
    impl PCINTPin for port::PC5 { pcmsk1 & (1 << 5) }
    impl PCINTPin for port::PC6 { pcmsk1 & (1 << 6) }

    impl PCINTPin for port::PD0 { pcmsk2 & (1 << 0) }
    impl PCINTPin for port::PD1 { pcmsk2 & (1 << 1) }
    impl PCINTPin for port::PD2 { pcmsk2 & (1 << 2) }
    impl PCINTPin for port::PD3 { pcmsk2 & (1 << 3) }
    impl PCINTPin for port::PD4 { pcmsk2 & (1 << 4) }
    impl PCINTPin for port::PD5 { pcmsk2 & (1 << 5) }
    impl PCINTPin for port::PD6 { pcmsk2 & (1 << 6) }
    impl PCINTPin for port::PD7 { pcmsk2 & (1 << 7) }
}