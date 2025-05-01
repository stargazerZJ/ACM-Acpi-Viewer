# Simple ACPI Hacker

[Lab 1.2](https://github.com/peterzheng98/os-2024-tutorial) of SJTU CS2952 Operating System.

## Quick Start

1. Setup qemu and other dependencies according to [tutorial](https://github.com/peterzheng98/os-2024-tutorial/releases/tag/v1.26).
2. Expected directory structure (you may modify [scripts/run.sh](scripts/run.sh) to change the path):
   ```
   .
   ├── kernel
   │   ├── edk2_out
   │   │   ├── OVMF_CODE.fd
   │   │   └── OVMF_VARS.fd
   │   └── uefi
   │       ├── acpi-hacker.efi   # symlink to compiled binary
   │       └── startup.nsh
   ├── ubuntu-cloud
   │   ├── ubuntu.img
   └── ACM-Acpi-Viewer
       └── README.md  # this project
   ```
3. Prepare a Ubuntu image according to [official docs](https://
documentation.ubuntu.com/public-images/en/latest/public-images-how-to/launch-qcow-with-qemu/). You may use a `seed.img` to set the password for the image. You may also need to install `acpidump` inside the image.
4. Edit `startup.nsh`:
```shell
fs0:                    # change to the filesystem where the acpi-viewer.efi is located
acpi-hacker.efi         # run the acpi-hacker.efi
fs1:                    # change to the filesystem where ubuntu is located
EFI\BOOT\BOOTX64.EFI    # run the ubuntu bootloader
```
5. Run qemu with cargo:
```bash
cargo run
```
6. Inside the Ubuntu VM, run `sudo acpidump` to verify that the OEM ID of table `FACP` is changed to `HACKED`.
7. To make the change persistant across reboots, configure the boot order to boot into EFI Internel Shell every time.