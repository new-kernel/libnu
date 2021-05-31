use core::fmt::{Arguments, Write, Result};
use novusk_syscalls::{syscall, SysCalls};

struct Writer;

impl Writer {
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        unsafe { syscall(SysCalls::Write, bytes, Default::default()) }
    }

    pub fn write_string(&mut self, string: &str) {
        self.write_bytes(string.as_bytes());
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_string(s);
        Ok(())
    }
}

pub fn _print(args: Arguments) -> Result {
    let mut writer = Writer;
    writer.write_fmt(args);
    Ok(())
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {$crate::text::output::_print(format_args!($($arg)*))};
}

#[macro_export]
macro_rules! println {
    () => ($crate::text::output::print!("\n"));
    ($($arg:tt)*) => {$crate::text::output::_print(format_args!($($arg)*))};
}
