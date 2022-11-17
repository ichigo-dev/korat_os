# Korat OS

## tools

- [bootimage](https://docs.rs/bootimage/latest/bootimage/)
	- Provides functions to create a bootable OS image from a kernel binary
	1. It compiles our kernel to an ELF file
	1. It compiles the `bootloader` dependency as a standalone executable
	1. It links the bytes of the kernel ELF file to the bootloader

## dependencies

- [bootloader](https://docs.rs/bootloader/latest/bootloader/)
	- An experimental x86_64 bootloader that works on both BIOS and UEFI systems
