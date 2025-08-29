
#[macro_export]
macro_rules! int_to_str
{
    ($int:expr, $length:expr) =>
    {
        {
            let mut n = $int;
            let mut i = $length;

            let mut buf: [u8; $length] = [b' '; $length];

            if n == 0
            {
                i -= 1;
                buf[i] = b'0';
            }
            else
            {
                while n > 0 && i > 0
                {
                    let digit = (n % 10) as u8;
                    i -= 1;
                    buf[i] = b'0' + digit;
                    n /= 10;
                }
            }

            buf
        }
    };
}