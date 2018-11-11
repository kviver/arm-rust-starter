#[macro_use]
pub mod uart_printf;

cfg_if! {
    if #[cfg(target_arch="arm")] {
        pub mod stm32_hal;
        pub use self::stm32_hal::*;
    } else if #[cfg(unix)] {
        pub mod stub_hal;
        pub use self::stub_hal::*;
    }
}

pub mod traits {

    #[derive(Debug)]
    pub enum PinState {
        Reset,
        Set,
    }

    pub trait Pin {
        fn write(&mut self, state: PinState);
        fn toggle(&mut self);
    }

    pub trait ADC {
        fn read_channel(&self, ch: u8) -> f32;
    }

    pub trait SPI {
        fn write(&self, bytes: &[u8]);
        fn read(&self, bytes: &mut [u8]);
        fn exchange(&self, tx_bytes: &[u8], rx_bytes: &mut [u8]);
    }
}
