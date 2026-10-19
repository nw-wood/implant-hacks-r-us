#!/usr/bin/env bash
# Copyright (C) 2026 NATHANYEL WOOD <nathanyelw@gmail.com>
# Licensed under the GNU General Public License v3.0 or later (GPL-3.0-or-later).
#
# ONE-COMMAND PUBLISHER FOR GITHUB (REPO + RELEASE + PAGES) & CRATES.IO

set -euo pipefail
export PATH="$HOME/.cargo/bin:$PATH"

echo "======================================================================"
echo " Publishing IMPLANT-HACKS-R-US v0.1.0 (GPL-3.0-or-later)"
echo "======================================================================"

# 1. Authenticate GitHub CLI if not already logged in
if ! gh auth status >/dev/null 2>&1; then
    echo ":: Step 1: Logging into GitHub CLI (nw-wood)..."
    gh auth login
fi

# 2. Create public repo https://github.com/nw-wood/implant-hacks-r-us and push main
if ! git remote get-url origin >/dev/null 2>&1; then
    echo ":: Creating public GitHub repository nw-wood/implant-hacks-r-us..."
    gh repo create nw-wood/implant-hacks-r-us \
        --public \
        --description "Dual-tone auditory matrix, Haas spatial dilation & psychoacoustic DSP tools (Hooked on Phon-et-ics)" \
        --source=. \
        --remote=origin \
        --push
else
    git push -u origin main
fi

# 3. Package & Upload Release Assets (Static PinePhone 64 ARM64, Linux x86_64, Web App, 14 WAV Pack)
echo ":: Creating release zip bundles..."
mkdir -p dist/releases
tar -czf dist/releases/implant-hacks-r-us-v0.1.0-linux-x86_64.tar.gz -C dist/linux-x86_64 implant-hacks-r-us
tar -czf dist/releases/implant-hacks-r-us-v0.1.0-pinephone64-rpi-arm64-static.tar.gz -C dist/pinephone64-rpi-arm64 implant-hacks-r-us-static
tar -czf dist/releases/implant-hacks-r-us-v0.1.0-webapp-and-wasm.tar.gz -C dist/web index.html implant_hacks_r_us.wasm
tar -czf dist/releases/implant-hacks-r-us-v0.1.0-wav-pack.tar.gz -C dist/wav-pack .
tar -czf dist/releases/implant-hacks-r-us-v0.1.0-baremetal-firmware.tar.gz -C dist/firmware .

if ! gh release view v0.1.0 >/dev/null 2>&1; then
    echo ":: Publishing GitHub Release v0.1.0 with all pre-compiled binaries & WAVs..."
    gh release create v0.1.0 \
        dist/releases/*.tar.gz \
        dist/web/index.html \
        --title "v0.1.0 — Hooked on Phon-et-ics All-in-One Release" \
        --notes "See DISCLAIMER.md before use. Includes static PinePhone 64 / RPi ARM64 binary, Linux x86_64 binary, standalone Web App (index.html + WASM), 14-file WAV pack, and Teensy 4.1 (i.MXRT1062) / ESP32 / RPi 5-minute de-escalation firmwares."
fi

# 4. Publish to crates.io (which automatically triggers docs.rs build!)
echo ":: Publishing crate to crates.io..."
if [ ! -f "$HOME/.cargo/credentials.toml" ] && [ -z "${CARGO_REGISTRY_TOKEN:-}" ]; then
    echo "   Paste your crates.io API token from https://crates.io/settings/tokens :"
    cargo login
fi
cargo publish

echo "======================================================================"
echo " ★ PUBLISHED SUCCESSFULLY!"
echo "   - GitHub Repo:    https://github.com/nw-wood/implant-hacks-r-us"
echo "   - GitHub Release: https://github.com/nw-wood/implant-hacks-r-us/releases/tag/v0.1.0"
echo "   - Crates.io:      https://crates.io/crates/implant-hacks-r-us"
echo "   - Docs.rs:        https://docs.rs/implant-hacks-r-us"
echo "======================================================================"
