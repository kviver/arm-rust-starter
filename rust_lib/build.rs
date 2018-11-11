extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn generate_cmsis_os_bindings(out_path:PathBuf) {
    let result_path = out_path.join("cmsis_os_bindings.rs");
    if result_path.is_file() {
        return
    }

    let includes = [
        "../Inc/",
        "../Middlewares/Third_Party/FreeRTOS/Source/include/",
        "../Middlewares/Third_Party/FreeRTOS/Source/portable/GCC/ARM_CM4F/",
        "/usr/lib/gcc/arm-none-eabi/4.9.3/include",
        "/usr/lib/gcc/arm-none-eabi/4.9.3/../../../arm-none-eabi/include",
    ];

    let builder = bindgen::Builder::default()
        .header("../Middlewares/Third_Party/FreeRTOS/Source/CMSIS_RTOS/cmsis_os.h")
        .use_core()

        // without this it generates `pub type __uint32_t = ::std::os::raw::c_uint;`
        .whitelist_recursively(false)

        .whitelist_type("osStatus")
        .rustified_enum("osStatus")

        .whitelist_type("osMutexId")
        .opaque_type("osMutexId")
        .whitelist_type("os_mutex_def")
        .whitelist_type("osMutexDef_t")
        .whitelist_function("osMutexCreate")
        .whitelist_function("osMutexWait")
        .whitelist_function("osMutexRelease")

        .whitelist_type("osSemaphoreId")
        .opaque_type("osSemaphoreId")
        .whitelist_type("os_semaphore_def")
        .whitelist_type("osSemaphoreDef_t")
        .whitelist_function("osSemaphoreCreate")
        .whitelist_function("osSemaphoreWait")
        .whitelist_function("osSemaphoreRelease")

        .whitelist_var("osWaitForever")
        .whitelist_function("osDelay")

        .clang_args(
            includes.iter().map(|include| format!("-I{}", include))
        )
        .clang_arg("--target=thumbv7em-none-eabi")
        .clang_arg("--verbose")
        .clang_arg("-nostdinc")
    ;

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(result_path)
        .expect("Couldn't write bindings!");
}

fn generate_stm32_hal_bindings(out_path:PathBuf) {
    let result_path = out_path.join("stm32_hal_bindings.rs");
    if result_path.is_file() {
        return
    }

    let includes = [
        "../Inc/",
        "../Drivers/STM32F4xx_HAL_Driver/Inc",
        "../Drivers/CMSIS/Include/",
        "../Drivers/CMSIS/Device/ST/STM32F4xx/Include/",
        "/usr/lib/gcc/arm-none-eabi/4.9.3/include",
        "/usr/lib/gcc/arm-none-eabi/4.9.3/../../../arm-none-eabi/include",
    ];

    let builder = bindgen::Builder::default()
        .header("../Drivers/STM32F4xx_HAL_Driver/Inc/stm32f4xx_hal.h")
        .use_core()

        // without this it generates `pub type __uint32_t = ::std::os::raw::c_uint;`
        .whitelist_recursively(false)

        .whitelist_type("HAL_StatusTypeDef")
        .rustified_enum("HAL_StatusTypeDef")

        .whitelist_type("GPIO_TypeDef")
        .opaque_type("GPIO_TypeDef")
        .whitelist_type("GPIO_PinState")
        .rustified_enum("GPIO_PinState")
        .whitelist_function("HAL_GPIO_WritePin")
        .whitelist_function("HAL_GPIO_TogglePin")

        .whitelist_type("UART_HandleTypeDef")
        .opaque_type("UART_HandleTypeDef")
        .whitelist_function("HAL_UART_Transmit_IT")

        .whitelist_type("SPI_HandleTypeDef")
        .opaque_type("SPI_HandleTypeDef")
        .whitelist_function("HAL_SPI_Transmit_IT")
        .whitelist_function("HAL_SPI_Receive_IT")
        .whitelist_function("HAL_SPI_TransmitReceive_IT")

        .clang_args(
            includes.iter().map(|include| format!("-I{}", include))
        )
        // todo this is a copy-paste from makefile
        .clang_arg("-DUSE_HAL_DRIVER")
        .clang_arg("-DSTM32F411xE")
        .clang_arg("--target=thumbv7em-none-eabi")
        .clang_arg("--verbose")
        .clang_arg("-nostdinc")
    ;

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(result_path)
        .expect("Couldn't write bindings!");
}

fn generate_glue_bindings(out_path:PathBuf) {
    let result_path = out_path.join("glue_bindings.rs");
    if result_path.is_file() {
        return
    }

    let includes = [
        "../Inc/",

        "../Drivers/STM32F4xx_HAL_Driver/Inc",
        "../Drivers/CMSIS/Include/",
        "../Drivers/CMSIS/Device/ST/STM32F4xx/Include/",

        "../Middlewares/Third_Party/FreeRTOS/Source/CMSIS_RTOS/",
        "../Middlewares/Third_Party/FreeRTOS/Source/include/",
        "../Middlewares/Third_Party/FreeRTOS/Source/portable/GCC/ARM_CM4F/",

        "/usr/lib/gcc/arm-none-eabi/4.9.3/include",
        "/usr/lib/gcc/arm-none-eabi/4.9.3/../../../arm-none-eabi/include",
    ];

    let builder = bindgen::Builder::default()
        .header("../Src/main.c")
        .use_core()

        // without this it generates `pub type __uint32_t = ::std::os::raw::c_uint;`
        .whitelist_recursively(false)

        .whitelist_var("HAL_LD2_GPIO_Port")
        .whitelist_var("HAL_LD2_Pin")

        .clang_args(
            includes.iter().map(|include| format!("-I{}", include))
        )
        // todo this is a copy-paste from makefile
        .clang_arg("-DUSE_HAL_DRIVER")
        .clang_arg("-DSTM32F446xx")
        .clang_arg("--target=thumbv7em-none-eabi")
        .clang_arg("--verbose")
        .clang_arg("-nostdinc")
    ;

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(result_path)
        .expect("Couldn't write bindings!");
}

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    if target == "thumbv7em-none-eabihf" {
        generate_cmsis_os_bindings(out_path.clone());
        generate_stm32_hal_bindings(out_path.clone());
        generate_glue_bindings(out_path.clone());
    }
}
