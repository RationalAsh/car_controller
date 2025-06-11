#!/bin/bash

# Echo everything
set -x

# Make the firmware directory if it doesn't exist
mkdir -p firmware

cargo build --release

# Dump ASM for the firmware for easier inspection and debugging
cargo objdump --release -- -d --no-show-raw-insn --print-imm-hex > firmware/firmware.asm

# Dump the vector table for the firmware for easier inspection and debugging
cargo objdump --release -- -s -j .vector_table > firmware/firmware.vector_table

# Create binary file
cargo objcopy --release -- -O binary firmware/firmware.bin

# Get binary size
# cargo size --release -- -Ax
cargo size --release