# tic-tac-toe-boot

Win (or draw) a game of tic-tac-toe vs an AI to boot your computer.


![project demo](./demo.gif)


The project compiles into an EFI application which should be added as a boot entry (or set to the default loader). It expects a compressed `bzImage` kernel at `/` to load after a player wins.

The kernel should be compiled with the EFI stub option enabled. Note that when running in QEMU or a similar virtual environment, framebuffer drivers should be compiled into the kernel to properly display output from the kernel.

Further, the bootloader does not yet support loading a separate initramfs -- the sample `bzImage` includes a simple, baked-in initramfs using a statically linked busyBox binary.

The AI is implemented using the naive minimax backtracking algorithm, which does not account for depth. Thus, drawing against the AI should be easier :).

## Building

```
rustup toolchain install nightly
rustup component add --toolchain nightly rust-src
cargo +nightly build --target x86_64-unknown-uefi
```

Built EFI application will be in `target/x86_64-unknown-uefi/debug/`.

## Running
The bootloader was tested using [`uefi-run`](https://github.com/Richard-W/uefi-run), which is a thin wrapper around QEMU using the OVMF BIOS port.

I have not run it on actual hardware yet, but I expect it to function approximately the same as it is simply an EFI application.
