use core::fmt::{Error, Write};

pub struct Console;

impl Write for Console {
    fn write_str(&mut self, out: &str) -> Result<(), Error> {
        unsafe {
            crate::rust_console_putbytes(out.as_ptr(), out.len());
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        let _ = write!(crate::console::Console, $($args)+);
    });
}

#[macro_export]
macro_rules! println {
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}
