# Simple ACPI Viewer

[Lab 1.1](https://github.com/peterzheng98/os-2024-tutorial) of SJTU CS2952 Operating System.

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
   │       ├── acpi-viewer.efi   # symlink to compiled binary
   │       └── startup.nsh
   └── ACM-Acpi-Viewer
       └── README.md  # this project
   ```
3. Edit `startup.nsh`:
```shell
fs0:              # change to the filesystem where the acpi-viewer.efi is located
acpi-viewer.efi   # run the acpi-viewer.efi
reset             # shut down the VM
```
4. Run qemu with cargo:
```bash
cargo run
```