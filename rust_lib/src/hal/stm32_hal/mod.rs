pub mod gpio_pin;
mod release;
// pub mod spi;
// pub mod stm32_uart;

mod bindings;

pub fn init_statics() {
    self::gpio_pin::init_pins();
    // self::stm32_uart::debug_uart_init_static();
    // self::spi::spi_init_static();
}

// pub use self::stm32_uart::{debug_uart_get};
