# Custom UEFI Runtime Service 

[Lab 1.3](https://github.com/peterzheng98/os-2024-tutorial) of SJTU CS2952 Operating System.

## Implementation Details

UEFI services are defined in the UEFI specification so there is no standard way to add a custom UEFI runtime service and use it in Linux.

On the rust side, we build a UEFI runtime driver, a kind of EFI program that gets executed when loaded, and it's code persists in memory even after OS take over.

The UEFI runtime driver defines the function to be executed using inline assembly (I tried rust functions but failed), and then store the function pointer as a UEFI variable.

On the Linux side, we first acquire the function pointer by reading the variable using UEFI runtime services. Then we set up proper memory mapping and permission.

Afterward, we add a new sysfs entry `/sys/firmware/efi/my_service/my_service` to the kernel, which is a file that can be read and written. The kernel module will read the function pointer from the UEFI variable and execute it when the file is written.

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
   │       ├── my-service.efi    # symlink to compiled binary
   │       ├── bzImage           # symlink to kernel image
   │       ├── initramfs.cpio    # symlink to initramfs
   │       └── startup.nsh
   ├── ubuntu-cloud
   │   └── ubuntu.img
   └── ACM-Acpi-Viewer
       └── README.md  # this project
   ```
3. Apply [patch](src/0001-uefi-runtime-service.patch) to [linux kernel v5.19.17](https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/snapshot/linux-5.19.17.tar.gz).
4. Build the kernel and initramfs according to [tutorial](https://github.com/peterzheng98/os-2024-tutorial/releases/tag/v1.26).
5. Copy the compiled kernel module `my_runtime_service.ko` to the root directory of the initramfs, and edit `init`:
```bash
#!/bin/busybox sh
mount -t proc none /proc
mount -t sysfs none /sys
mount -t tmpfs none /tmp
mount -t devtmpfs none /dev
# mount /dev/sda1 /mnt/efi
echo "Hello Linux!"
insmod my_runtime_service.ko
echo 2 > /sys/firmware/efi/my_service/my_service
cat /sys/firmware/efi/my_service/my_service
sh
rmmod my_runtime_service
poweroff -f
```
6. Edit `startup.nsh`:
```shell
fs0:                    # change to the filesystem where the acpi-viewer.efi is located
load my-service.efi     # load the UEFI runtime service
# boot the kernel
bzImage initrd=initramfs.cpio.gz init=/init console=ttyS0
reset                   # in case the kernel does not boot
```
7. Build the UEFI runtime service:
```bash
cargo build
```
8. Now run qemu. No one-click runner script is provided, but a reference [script](scripts/efi.sh) is provided, read and edit it to fit your need.