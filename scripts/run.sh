#!/usr/bin/env bash

# This script runs the UEFI application in QEMU with OVMF firmware.

script_dir=$(dirname "$(realpath "$0")")
cd "$script_dir"/../.. || exit 1  # Go to the parent directory of the project root

qemu-system-x86_64 \
  -machine q35 \
  -m 8G \
  -smp 4 \
  -drive if=pflash,format=raw,unit=0,file="kernel/edk2_out/OVMF_CODE.fd",readonly=on \
  -drive if=pflash,format=raw,unit=1,file="kernel/edk2_out/OVMF_VARS.fd" \
  -drive file=fat:rw:"kernel/uefi",format=raw,if=ide,index=0 \
  -nographic \
  -net none \
  -no-reboot \
  -serial mon:stdio