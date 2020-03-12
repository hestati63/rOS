mod cga; // Color Graphics Adapter
mod lpt; // Line Print Terminal
mod serial; // Serial I/O

use crate::locking::SpinLock;
use arch::Virtual;
use cga::CGA;
use core::fmt;
use lpt::LPT;
use serial::Serial;

struct Console {
    ser: bool,
    cga: CGA,
}

impl Console {
    pub fn new(kern_base: Virtual) -> Console {
        Console {
            ser: Serial::init().is_ok(),
            cga: CGA::init(kern_base),
        }
    }

    fn puts(&mut self, s: &str) {
        for byte in s.chars() {
            // FIXME: fix CGA later
            self.cga.putc(cga::VGAChar {
                ascii: byte as u8,
                color: Default::default(),
            });
            LPT::putc(byte as u8);
            Serial::putc(byte as u8);
            if self.ser {}
        }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        Console::puts(self, s);
        Ok(())
    }
}

static mut CONSOLE: Option<SpinLock<Console>> = None;

#[link_section = ".init.text"]
#[no_mangle]
pub fn init(kern_base: Virtual) {
    unsafe {
        CONSOLE = Some(SpinLock::new(Console::new(kern_base)));
    }
}

/// Print with lock.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::dev::tty::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        CONSOLE.as_mut().unwrap().borrow().write_fmt(args).unwrap();
    }
}

/// Print without lock.
/// The primary use of this is panic handler.
/// Panic can occurs during print-locked context.
#[macro_export]
macro_rules! print_unlocked {
    ($($arg:tt)*) => ($crate::dev::tty::_print_unlocked(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println_unlocked {
    () => ($crate::print_unlocked!("\n"));
    ($($arg:tt)*) => ($crate::print_unlocked!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print_unlocked(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        CONSOLE.as_mut().unwrap().steal().write_fmt(args).unwrap();
    }
}
