default: target/wimp,ff8

target/wimp,ff8: target/arm-none-eabi/release/wimp
	arm-none-eabi-objcopy -O binary target/arm-none-eabi/release/wimp target/wimp,ff8

target/arm-none-eabi/release/wimp: PHONY
	cargo build --release

PHONY:
