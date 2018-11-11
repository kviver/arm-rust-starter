# STM32+Rust FFI starter kit

## Using docker
Run `docker-compose up -d`, then use `docker-compose exec dev bash` to get a shell with all the tools installed.

## How to build this

1. Create new STM32CubeMX makefile project with freertos into project dir.
2. Add to makefile after `all` target:

```
# libraries
RUST_LIB_NAME = rust_lib
LIBS += -l$(RUST_LIB_NAME) 
RUST_LIB_SRC = rust_lib
RUST_LIB_BUILD = $(RUST_LIB_SRC)/target/thumbv7em-none-eabihf/debug
LIBDIR += -L$(RUST_LIB_BUILD)

.PHONY: $(RUST_LIB_BUILD)/lib$(RUST_LIB_NAME).a

$(RUST_LIB_BUILD)/lib$(RUST_LIB_NAME).a:
	cd $(RUST_LIB_SRC) && xargo build
```
add `$(RUST_LIB_BUILD)/lib$(RUST_LIB_NAME).a` dependency to `$(BUILD_DIR)/$(TARGET).elf:`

add `cd $(RUST_LIB_SRC) && cargo clean` to `make clean` target

3. also add app_task() and app_static_init() call to main.c, GPIO port/pin macro static exports, device pointers getter (get_huart2, for example)
```
void rust_main_task();
```
```
GPIO_TypeDef* HAL_LD2_GPIO_Port = LD2_GPIO_Port;
uint16_t HAL_LD2_Pin = LD2_Pin;
```
```
UART_HandleTypeDef* get_huart2(){
  return &huart2;
}
```

4. don't forget binpath in makefile (`BINPATH = /usr/bin/`, for example)

5. Compile
`make`

6. Flash uС
`./deploy.sh`

To debug Rust code use 
`RUST_GDB=arm-none-eabi-gdb rust-gdb <path to .elf file>`
