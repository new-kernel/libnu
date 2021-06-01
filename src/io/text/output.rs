use core::fmt::{Arguments, Write, Result};
use novusk_syscalls::{syscall, SysCalls};

/// The Writer struct lets you safely print with different types, like &[u8] and &str.
pub struct Writer;

/// write_bytes is used for writing &[u8], write_string is used for writing &str and is used in
/// write_str for write_fmt.
///
/// ### Example:
///
/// let mut writer = Writer;
/// writer.write_bytes(b"Hello, World!\n");
/// writer.write_string("Hello, World!\n");
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

/// The _print function is the function used in print! and println!
pub fn _print(args: Arguments) -> Result {
    let mut writer = Writer;
    writer.write_fmt(args);
    Ok(())
}

/// print! is just like the Rust stdlib print!, it just prints formatted text
///
/// ### Example:
///
/// print!("This, {} formatted {}\n", "is", "text");
///
/// print!("ya");
///
/// print!("y!");
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {$crate::text::output::_print(format_args!($($arg)*))};
}

/// println! is also just like the Rust stdlib println!, it prints formatted text and a new line.
///
/// ### Example:
///
/// println!("No new line needed!");
#[macro_export]
macro_rules! println {
    () => ($crate::text::output::print!("\n"));
    ($($arg:tt)*) => {$crate::text::output::_print(format_args!($($arg)*))};
}
