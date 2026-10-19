#!/usr/bin/env bash
# Copyright (C) 2026 NATHANYEL WOOD <nathanyelw@gmail.com>
# Licensed under the GNU General Public License v3.0 or later (GPL-3.0-or-later).
#
# ONE-COMMAND MULTI-TARGET BUILDER FOR IMPLANT-HACKS-R-US
# Builds self-contained zero-dependency deliverables in dist/ for:
#   1. Web App (Single copy-pasteable HTML file + WASM module)
#   2. Linux Desktop (x86_64-unknown-linux-gnu)
#   3. PinePhone 64 & Raspberry Pi 64-bit Static Binary (aarch64-unknown-linux-musl)
#   4. PinePhone 64 & Raspberry Pi 64-bit glibc Binary (aarch64-unknown-linux-gnu)
#   5. Pre-rendered WAV Pack (all 14 WAVs ready to copy onto any phone/SD card)

set -euo pipefail
export PATH="$HOME/.cargo/bin:$PATH"

echo "======================================================================"
echo " Building IMPLANT-HACKS-R-US (Hooked on Phon-et-ics All-in-One)"
echo "======================================================================"

mkdir -p dist/web dist/linux-x86_64 dist/pinephone64-rpi-arm64 dist/wav-pack dist/firmware

echo "[1/5] Building Host Linux x86_64 Binary..."
cargo build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/implant-hacks-r-us dist/linux-x86_64/

echo "[2/5] Building Static ARM64 Binary for PinePhone 64 & Raspberry Pi (aarch64-unknown-linux-musl)..."
cargo zigbuild --release --target aarch64-unknown-linux-musl
cp target/aarch64-unknown-linux-musl/release/implant-hacks-r-us dist/pinephone64-rpi-arm64/implant-hacks-r-us-static

echo "[3/5] Building WebAssembly DSP Core (wasm32-unknown-unknown) & Standalone Web App..."
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/implant_hacks_r_us.wasm dist/web/
cp web/index.html dist/web/index.html

echo "[4/5] Copying Embedded Metal Sources (ESP32, Teensy 4.1 i.MXRT1062, RPi GPIO)..."
cp -r firmware/* dist/firmware/

echo "[5/5] Generating Complete 14-File Phonics & Defense WAV Pack..."
./dist/linux-x86_64/implant-hacks-r-us --export-all dist/wav-pack

echo "======================================================================"
echo " ★ GOLD STAR! All targets built successfully in ./dist/:"
echo "   - dist/web/index.html                           (Zero-install Web App for Android/PinePhone/Desktop)"
echo "   - dist/web/implant_hacks_r_us.wasm              (Compiled Rust WASM DSP Library)"
echo "   - dist/linux-x86_64/implant-hacks-r-us          (Single-file x86_64 Studio + CLI + Switch Daemon)"
echo "   - dist/pinephone64-rpi-arm64/implant-hacks-r-us-static (100% Static ARM64 Binary for PinePhone 64 & RPi)"
echo "   - dist/firmware/                                (Teensy 4.1 i.MXRT1062, ESP32, RPi 5-min De-escalator)"
echo "   - dist/wav-pack/                                (All 14 ready-to-play .WAV files)"
echo "======================================================================"
