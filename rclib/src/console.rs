use core::fmt::{self, Write};

struct Stdout;

const STDOUT: usize = 1;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        crate::write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn print(arg: fmt::Arguments) {
    Stdout.write_fmt(arg).unwrap()
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($args: tt)+)?) => {
        $crate::console::print(format_args($fmt $(, $($args)+)?))
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($args: tt)+)?) => {
        $crate::console::print(format_args(concat!($fmt, "\n") $(, $($args)+)?))
    };
}
