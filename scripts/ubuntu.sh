#!/usr/bin/env bash

# This script runs the UEFI application in QEMU with OVMF firmware.

script_dir=$(dirname "$(realpath "$0")")
cd "$script_dir"/../.. || exit 1  # Go to the parent directory of the project root

qemu-system-x86_64 \
  -machine q35 \
  -m 8G \
  -smp 4 \
  -snapshot \
  -drive if=pflash,format=raw,unit=0,file="kernel/edk2_out/OVMF_CODE.fd",readonly=on \
  -drive if=pflash,format=raw,unit=1,file="kernel/edk2_out/OVMF_VARS.fd" \
  -drive file="ubuntu-cloud/ubuntu.img",format=qcow2,if=virtio \
  -drive file=fat:ro:"kernel/uefi",format=raw,if=ide,index=1 \
  -nographic \
  -net none \
  -serial mon:stdio