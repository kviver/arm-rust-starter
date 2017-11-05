# Hermitool firmware sandbox

## Using docker
Run `docker-compose up -d`, then use `docker-compose exec dev bash` to get a shell with all the tools installed.

## Using rust
main c-code using simple function from Rust, having full path to it in Makefile
Go to `rust_lib/`

Create new STM32CubeMX into project dir.
Add to makefile:

```
# libraries
RUST_LIB_NAME = rust_lib
LIBS += -l$(RUST_LIB_NAME) 
RUST_LIB_SRC = rust_lib
RUST_LIB_BUILD = $(RUST_LIB_SRC)/target/thumbv7em-none-eabihf/debug
LIBDIR += -L$(RUST_LIB_BUILD)
LDFLAGS = $(MCU) -specs=nano.specs -T$(LDSCRIPT) $(LIBDIR) $(LIBS) -Wl,-Map=$(BUILD_DIR)/$(TARGET).map,--cref -Wl,--gc-sections

.PHONY: $(RUST_LIB_BUILD)/lib$(RUST_LIB_NAME).a

$(BUILD_DIR)/$(TARGET).elf: $(RUST_LIB_BUILD)/lib$(RUST_LIB_NAME).a $(OBJECTS) Makefile
	$(CC) $(OBJECTS) $(LDFLAGS) -o $@
	$(SZ) $@
```

Compile library

`make`

To debug Rust code use 
`RUST_GDB=arm-none-eabi-gdb rust-gdb <path to .elf file>`
