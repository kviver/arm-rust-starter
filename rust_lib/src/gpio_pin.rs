use hal::{
    HAL_GPIO_TogglePin,
    HAL_GPIO_WritePin,
    GPIO_TypeDef,
    GPIO_PinState,
};

pub struct GPIOPin {
    pub gpio_port: *mut GPIO_TypeDef,
    pub pin: u16,
}

impl GPIOPin {
    pub fn write(&mut self, state: GPIO_PinState) {
        unsafe { HAL_GPIO_WritePin(self.gpio_port, self.pin, state) };
    }

    pub fn toggle(&mut self) {
        unsafe { HAL_GPIO_TogglePin(self.gpio_port, self.pin) };
    }
}