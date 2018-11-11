use fmt;
use os::Mutex;
use peripheral::Static;
use colored::*;

pub fn init_statics() {
    gpio_pin::init_gpio_statics();
    spi::init_spi_statics();
    init_uart_statics();
}

pub struct UART<'a> {
    name: &'a str
}

impl<'a> fmt::Write for UART<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        print!("[{} {}] {}", "UART".green(), self.name.green(), s);
        Ok(())
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        print!("[{} {}] {}", "UART".green(), self.name.green(), args);
        Ok(())
    }
}

pub static DEBUG_UART: Static<Mutex<UART>> = Static::new();

pub fn init_uart_statics() {
    DEBUG_UART.init(Mutex::new(UART {
        name: "DEBUG_UART",
    }));
}

pub fn debug_uart_get() -> &'static Mutex<UART<'static>> {
    DEBUG_UART.get()
}

pub mod gpio_pin {
    use hal::traits::{Pin, PinState};
    use os::Mutex;
    use peripheral::Static;

    pub struct GPIOPin<'a> {
        name: &'a str,
        state: PinState
    }

    impl<'a> GPIOPin<'a> {
        pub fn new(name:&str) -> GPIOPin {
            GPIOPin { name: name, state: PinState::Reset }
        }
    }

    impl<'a> Pin for GPIOPin<'a> {
        fn write(&mut self, state: PinState) {
            println!("[GPIO {}] write {:?}", self.name, state);
            self.state = state;
        }

        fn toggle(&mut self) {
            println!("[GPIO {}] toggle to {:?}", self.name, self.state);
            self.state = match self.state {
                PinState::Reset => PinState::Set,
                PinState::Set => PinState::Reset,
            }
        }
    }

    pub static LD2_PIN: Static<Mutex<GPIOPin>> = Static::new();

    pub static CONTROL_LATCH_PIN: Static<Mutex<GPIOPin>> = Static::new();

    pub fn init_gpio_statics() {
        LD2_PIN.init(Mutex::new(GPIOPin {
            name: "LD2_PIN",
            state: PinState::Reset
        }));

        CONTROL_LATCH_PIN.init(Mutex::new(GPIOPin {
            name: "CONTROL_LATCH_PIN",
            state: PinState::Reset
        }));
    }
}

pub mod spi {
    use hex_slice::AsHex;

    use hal::traits::{SPI as SPITrait};
    use os::Mutex;
    use peripheral::Static;

    pub struct SPI<'a> {
        name: &'a str
    }

    impl<'a> SPITrait for SPI<'a> {
        fn write(&self, bytes: &[u8]) {
            println!("[SPI {}] write {} bytes: {:#x}", self.name, bytes.len(), bytes.as_hex());
        }
        fn read(&self, bytes: &mut [u8]) {
            //TODO read some stub data
            println!("[SPI {}] read {} bytes", self.name, bytes.len());
        }
        fn exchange(&self, tx_bytes: &[u8], rx_bytes: &mut [u8]) {
            assert_eq!(tx_bytes.len(), rx_bytes.len(), "Exchanging TX and RX buffer length mismatch");
            //TODO read some stub data
            println!("[SPI {}] exchange {} bytes, write: {:#x}", self.name, tx_bytes.len(), tx_bytes.as_hex());
        }
    }

    pub static CR_SPI: Static<Mutex<SPI>> = Static::new();
    pub static ADC_SPI: Static<Mutex<SPI>> = Static::new();

    pub fn init_spi_statics() {
        CR_SPI.init(
            Mutex::new(
                SPI { name: "CR_SPI" }
            )
        );
        ADC_SPI.init(
            Mutex::new(
                SPI { name: "ADC_SPI" }
            )
        );
    }
}
