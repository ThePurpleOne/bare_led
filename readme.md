# Bare metal Rust Blink

## Run on real hardware

Need : 
```bash
rustup target add armv7a-none-eabi
```

### Compile code into ELF: 
```bash
cargo build --release
```

### Flatten into binary
```bash
arm-none-eabi-objcopy -O binary ./target/armv7a-none-eabi/release/led ./kernel.img
```

### Put it on SD
Put the `to_push/kernel.img` on the SD card alongside with:
- bootcode.bin
- fixup.dat
- start.elf

## Simulate it
Need : 
```bash
rustup target add aarch64-unknown-none
```

### QEMU Simulating
We can test out things with qemu:
```bash
qemu-system-aarch64 -M raspi3b -serial stdio -kernel to_push/kernel.img
```

## Sources
[Sourceware : Linker script explanation](https://sourceware.org/binutils/docs/ld/Scripts.html#Scripts)
[BCM 2837 Datasheet](https://cs140e.sergio.bz/docs/BCM2837-ARM-Peripherals.pdf)
[Interactive Pinout](https://pinout.xyz/)
