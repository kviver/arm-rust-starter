pub trait UART {
    fn write(&self, s: &str);
}

#[macro_export]
macro_rules! impl_uart_write {
    ($uart:ty) => {
        impl $crate::fmt::Write for $uart {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.write(s);
                return Ok(());
            }
        }
    };
}

#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        {
            let uart_local = debug_uart_get();
            let mut uart = uart_local.lock();
            write!(*uart, $($arg)*).unwrap()
        }
    };
}

#[macro_export]
macro_rules! debug_println {
    () => (debug_print!("\n"));
    ($fmt:expr) => (debug_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (debug_print!(concat!($fmt, "\n"), $($arg)*));
}
