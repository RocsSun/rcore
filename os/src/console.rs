use core::fmt::{Arguments, Write};

use crate::sbi::console_putchar;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        s.chars().for_each(|x| {
            console_putchar(x as usize);
        });
        Ok(())
    }
}

pub fn print(arg: Arguments) {
    Stdout.write_fmt(arg).unwrap();
}

#[macro_export]
macro_rules! print {
    ($ff: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(ff $(, $($arg)+)?))
    };
}

#[macro_export]
macro_rules! println {
    ($ff: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($ff, "\n") $(, $($arg)+)?))
    };
}
