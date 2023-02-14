# Bare metal Rust Blink

## Compile code into ELF: 
```bash
cargo build --release
```

## Flatten into binary
```bash
arm-none-eabi-objcopy -O binary ./target/armv7a-none-eabi/release/led ./kernel.img
```

## Sources
(Sourceware : Linker script explanation)[https://sourceware.org/binutils/docs/ld/Scripts.html#Scripts]

