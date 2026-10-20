#!/usr/bin/env bash
# Copyright (C) 2026 NATHANYEL WOOD <nathanyelw@gmail.com>
# Licensed under the GNU General Public License v3.0 or later (GPL-3.0-or-later).
#
# ONE-COMMAND PUBLISHER FOR GITHUB (REPO + RELEASE + PAGES) & CRATES.IO

set -euo pipefail
export PATH="$HOME/.cargo/bin:$PATH"

VERSION=$(grep -E '^version\s*=' Cargo.toml | head -n1 | sed -E 's/.*"([^"]+)".*/\1/')

echo "======================================================================"
echo " Publishing IMPLANT-HACKS-R-US v${VERSION} (GPL-3.0-or-later)"
echo "======================================================================"

# 1. Authenticate GitHub CLI if not already logged in
if ! gh auth status >/dev/null 2>&1; then
    echo ":: Step 1: Logging into GitHub CLI (nw-wood)..."
    gh auth login
fi

# 2. Package Release Assets (Static PinePhone 64 ARM64, Linux x86_64, Web App, 16 WAV Pack)
echo ":: Creating release tar.gz bundles for v${VERSION}..."
mkdir -p dist/releases
rm -f dist/releases/*.tar.gz
tar -czf "dist/releases/implant-hacks-r-us-v${VERSION}-linux-x86_64.tar.gz" -C dist/linux-x86_64 implant-hacks-r-us
tar -czf "dist/releases/implant-hacks-r-us-v${VERSION}-pinephone64-rpi-arm64-static.tar.gz" -C dist/pinephone64-rpi-arm64 implant-hacks-r-us-static
tar -czf "dist/releases/implant-hacks-r-us-v${VERSION}-webapp-and-wasm.tar.gz" -C dist/web index.html implant_hacks_r_us.wasm
tar -czf "dist/releases/implant-hacks-r-us-v${VERSION}-wav-pack.tar.gz" -C dist/wav-pack .
tar -czf "dist/releases/implant-hacks-r-us-v${VERSION}-baremetal-firmware.tar.gz" -C dist/firmware .

# Commit any updated release bundles before pushing & publishing
if [ -n "$(git status --porcelain)" ]; then
    git add -A
    git commit -m "chore(release): package v${VERSION} release bundles"
fi

# 3. Create or push public repo https://github.com/nw-wood/implant-hacks-r-us
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

# 4. Create or Update GitHub Release for v${VERSION}
if ! gh release view "v${VERSION}" >/dev/null 2>&1; then
    echo ":: Publishing GitHub Release v${VERSION} with all pre-compiled binaries & WAVs..."
    gh release create "v${VERSION}" \
        dist/releases/*.tar.gz \
        dist/web/index.html \
        --title "v${VERSION} — Hooked on Phon-et-ics All-in-One Release" \
        --notes "See DISCLAIMER.md before use. Includes Tier 5 (Secret Black Workbook 220+7200Hz rails + Grey Noise), Tier 6 ('I don't care anymore eat puppies' 1000Hz constant wave), Keypad Loop Player, static PinePhone 64 / RPi ARM64 binary, Linux x86_64 binary, standalone Web App (index.html + WASM), 16-file WAV pack, and Teensy 4.1 (i.MXRT1062) / ESP32 / RPi 5-minute de-escalation firmwares."
else
    echo ":: Updating assets on existing GitHub Release v${VERSION}..."
    gh release upload "v${VERSION}" dist/releases/*.tar.gz dist/web/index.html --clobber
fi

# 5. Publish to crates.io (crates.io versions are immutable; bumps version in Cargo.toml)
echo ":: Publishing v${VERSION} to crates.io..."
if [ ! -f "$HOME/.cargo/credentials.toml" ] && [ -z "${CARGO_REGISTRY_TOKEN:-}" ]; then
    echo "   Paste your crates.io API token from https://crates.io/settings/tokens :"
    cargo login
fi
cargo publish --allow-dirty

echo "======================================================================"
echo " ★ PUBLISHED v${VERSION} SUCCESSFULLY!"
echo "   - GitHub Repo:    https://github.com/nw-wood/implant-hacks-r-us"
echo "   - GitHub Release: https://github.com/nw-wood/implant-hacks-r-us/releases/tag/v${VERSION}"
echo "   - Crates.io:      https://crates.io/crates/implant-hacks-r-us/${VERSION}"
echo "   - Docs.rs:        https://docs.rs/implant-hacks-r-us/${VERSION}"
echo "======================================================================"
