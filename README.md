# tic-tac-toe-boot

Win tic-tac-toe vs an AI to boot your computer.

## Building

```
rustup toolchain install nightly
rustup component add --toolchain nightly rust-src
cargo +nightly build --target x86_64-unknown-uefi
```

Built EFI application will be in `target/x86_64-unknown-uefi/debug/`.

## Running
Haven't tried on actual hardware, but I've been using [`uefi-run`](https://github.com/Richard-W/uefi-run).
