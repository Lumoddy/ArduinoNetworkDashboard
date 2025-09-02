
macro_rules! push_x_impl
{
    ($vis:vis fn $name:ident $num:ty as signed $larger:ty;) =>
    {
        $vis fn $name<Char: Clone>(
            value: $num,
            alphabet: &[Char],
            negative: Char,
            mut push: impl FnMut(Char))
        {
            let mut value = value as $larger;

            if value < 0
            {
                push(negative);
                value = -value;
            }

            let alphabet_length = alphabet.len() as $larger;

            loop
            {
                let mut digit_value = value;
                let mut scaler: $larger = 1;

                while digit_value >= alphabet_length
                {
                    digit_value /= alphabet_length;
                    scaler *= alphabet_length;
                }

                push(alphabet[digit_value as usize].clone());

                value -= scaler * digit_value;

                if value == 0 { break };
            }
        }
    };
    ($vis:vis fn $name:ident $num:ty as unsigned $larger:ty;) =>
    {
        $vis fn $name<Char: Clone>(
            value: $num,
            alphabet: &[Char],
            mut push: impl FnMut(Char))
        {
            let mut value = value as $larger;

            let alphabet_length = alphabet.len() as $larger;

            loop
            {
                let mut digit_value = value;
                let mut scaler: $larger = 1;

                while digit_value >= alphabet_length
                {
                    digit_value /= alphabet_length;
                    scaler *= alphabet_length;
                }

                push(alphabet[digit_value as usize].clone());

                value -= scaler * digit_value;

                if value == 0 { break };
            }
        }
    };
    (
        $first_vis:vis fn $first_name:ident $first_num:ty as $first_sign:ident $first_larger:ty;
        $($rest_vis:vis fn $rest_name:ident $rest_num:ty as $rest_sign:ident $rest_larger:ty;)+
    ) =>
    {
        push_x_impl!
        {
            $first_vis fn $first_name $first_num as $first_sign $first_larger;
        }
        $(
            push_x_impl!
            {
                $rest_vis fn $rest_name $rest_num as $rest_sign $rest_larger;
            }
        )+
    };
}

push_x_impl!
{
    pub fn push_u8 u8 as unsigned u16;
    pub fn push_i8 i8 as signed i16;
    pub fn push_u16 u16 as unsigned u32;
    pub fn push_i16 i16 as signed i32;
    pub fn push_u32 u32 as unsigned u64;
    pub fn push_i32 i32 as signed i64;
    pub fn push_u64 u64 as unsigned u128;
    pub fn push_i64 i64 as signed i128;
    pub fn push_usize usize as unsigned u32;
    pub fn push_isize isize as signed i32;
}