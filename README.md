# Linux-Kernel-Development
Sandboxed Redox OS and Hacked the linux kernel to add a syscall.

## Instructions for Redox OS
- First install the rust based architecture.
- Clone the repostiory [Redox-OS](https://gitlab.redox-os.org/redox-os/redox.git).
- Compile and build the kernel using the instructions on documentation of official redox os distribution.
- Run `git submodule update --recursive --init` to update the submodules.
- Run qemu emulator to simulate the OS with and without kernel depending on how the flag set.
- `kvm=no` for simulation without kernel and default is with kernel
- Run `make qemu kvm=no vga=no` to simulate the OS without kernel and graphics (Terminal only).


## Instructions for Linux Kernel

- First download the linux source code.
- Replace the file `kernel/sys.c` in it with the file provided in the zip.
- Replace the file `arch/x86/entry/syscalls/syscall_64.tbl` in it with the `syscall_64.tbl` provided in this zip.
- Now compile and build the kernel using the instructions on the documentation of official linux kernel.
- Compile and execute the `testing.c` script after booting into the QEMU emulator.
- print the log using `"dmesg | tail"` command to see the result :)
