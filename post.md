Hello,
i'm doing some Bare metal rust on a Raspi but when i'm trying to build it seems i don't have the right toolchain.

I've already added to `.cargo/config`:
```toml
[build]
target = "armv7a-none-eabi"
```

And i get the following error when building:
```bash
error[E0463]: can't find crate for `core`
  |
  = note: the `armv7a-none-eabi` target may not be installed
  = help: consider downloading the target with `rustup target add armv7a-none-eabi`
```

I tried to download it with
```bash
rustup toolchain install stable-armv7a-none-eabi
info: syncing channel updates for 'stable-armv7a-none-eabi'
info: latest update on 2023-01-10, rust version 1.66.1 (90743e729 2023-01-10)
error: target 'armv7a-none-eabi' not found in channel.  Perhaps check https://doc.rust-lang.org/nightly/rustc/platform-support.html for available targets
```

I also tried to download it with
```bash
rustup toolchain add armv7a-none-eabi
error: invalid toolchain name: 'armv7a-none-eabi'
```

Even though it's available on the cited website:
https://doc.rust-lang.org/nightly/rustc/platform-support.html
