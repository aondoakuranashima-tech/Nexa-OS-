# Nexa OS

Nexa OS is a Rust-powered operating system targeting 64-bit RISC-V.

## Current target

- Architecture: RV64GC
- Rust: nightly, `no_std`
- Platform: QEMU `virt`
- Firmware: OpenSBI via QEMU's default BIOS
- Kernel entry: RISC-V assembly → Rust
- Console: QEMU UART at `0x10000000`

The QEMU `virt` machine is a standard RISC-V development platform, and OpenSBI provides the SBI layer between machine-mode firmware and supervisor-mode operating systems. citeturn0search0turn0search12

## Project status

### v0.1 — Kernel foundation

- [x] Rust bare-metal target configuration
- [x] RISC-V linker script
- [x] Kernel entry point
- [x] Initial kernel stack
- [x] BSS initialization
- [x] UART console output
- [x] Rust panic handler
- [ ] Trap/exception handling
- [ ] Timer initialization
- [ ] Physical memory allocator
- [ ] Virtual memory
- [ ] Kernel heap
- [ ] Task switching
- [ ] Scheduler
- [ ] User mode
- [ ] System calls

## Build

Install the RISC-V Rust target and QEMU:

```bash
rustup target add riscv64gc-unknown-none-elf
```

Then:

```bash
cargo build
```

## Run

```bash
make run
```

Expected output:

```text
NEXA OS
========
Architecture: RISC-V 64
Kernel: Rust
Boot: OK
HART: 0

nexa>
```

## Architecture direction

Nexa OS will keep the kernel core independent from platform-specific code. RISC-V support lives at the architecture/platform boundary so that future RISC-V hardware targets can be added without rewriting the kernel core.

The initial implementation deliberately stays small. No filesystem, networking stack, GUI, package manager, or application framework will be added until the kernel has reliable memory management, traps, scheduling, and user-space execution.
