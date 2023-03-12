TOOLCHAIN_HARDWARE := armv7a-none-eabi
TOOLCHAIN_SIM := aarch64-unknown-none


all :
	@echo "> make hardware : compile into a flat binary ./to_push/kernel.img"
	@echo "> make sim      : compile and run it in qemu"

# Compile and flatten, only need to put into sd
hardware:
	cargo build --release --target=$(TOOLCHAIN_HARDWARE)
# Flatten the binary from elf to raw binary 
	arm-none-eabi-objcopy -O binary ./target/$(TOOLCHAIN_HARDWARE)/release/led ./to_push/kernel.img

# Compile and run in qemu
sim:
	cargo build --release --target=$(TOOLCHAIN_SIM)
	qemu-system-aarch64 -M raspi3b -serial vc -serial stdio -kernel ./target/$(TOOLCHAIN_SIM)/release/led