#!/usr/bin/env bash

set -e

# Create bin file
riscv64-unknown-elf-objcopy $1 -O binary $1.bin

# Program iCEBreaker-bitsy
dfu-util -a 1 -R -d 1d50:6146 -D $1.bin
