# Korat OS

## tips

### QEMU

QEMU is configured to be used with `curses` option. This allows you to view the 
virtual machine's console directly in a virtual terminal. This is useful for 
development in a Docker environment.

![exec `cargo run`](/note/assets/qemu_run.mp4)

To exit the emulator, press `alt` + `2` to connect to the QEMU monitor and press 
`q`.

## tools

- [QEMU](https://www.qemu.org/)
	- A generic and open source machine emulator and virtualizer

- [bootimage](https://docs.rs/bootimage/latest/bootimage/)
	- Provides functions to create a bootable OS image from a kernel binary
	1. It compiles our kernel to an ELF file
	1. It compiles the `bootloader` dependency as a standalone executable
	1. It links the bytes of the kernel ELF file to the bootloader

## dependencies

- [bootloader](https://docs.rs/bootloader/latest/bootloader/)
	- An experimental x86_64 bootloader that works on both BIOS and UEFI systems
