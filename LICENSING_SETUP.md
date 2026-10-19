THIS WAS AUTOMATICALLY GENERATED - USE NATHANYEL WOOD FOR ALL REFERENCES, EMAIL IS NATHANYELW@GMAIL.COM
MAKE SURE IT MATCHES THE ABOVE THROUGHOUT WHEN SETTING UP

Step 1: Create the Master LICENSE File

In the root of your project directory, create a file named LICENSE (or COPYING) containing the official GPLv3 plain text.

From your terminal, download it directly:
Bash

curl -sSL https://www.gnu.org/licenses/gpl-3.0.txt -o LICENSE

Step 2: Add the Standard Notice to Source Files

The Free Software Foundation requires a concise notice block at the top of each source file.

In Rust files (e.g., src/main.rs, src/lib.rs), place this at the very top:
Rust

// Copyright (C) 2026 NATHANYEL WOOD
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

fn main() {
    // Project entry point
}

Note: You can use your legal name, or both (e.g., Legal Name (_/W\/W\/W\/W\/W\/W\/\_)). Copyright applies to the author regardless of pseudonymity, provided identity can be established if challenged.
Step 3: Declare the License in Cargo

In your Cargo.toml, specify the license using its standard SPDX identifier:
Ini, TOML

[package]
name = "implant-hacks-r-us"
version = "0.1.0"
edition = "2021"
authors = ["Nathanyel Wood <nathanyelw@gmail.com>"]
license = "GPL-3.0-or-later"
description = "Dual-tone auditory matrix and psychoacoustic DSP tools"

    Use "GPL-3.0-or-later" if you included the "either version 3 of the License, or (at your option) any later version" clause.

    Use "GPL-3.0-only" if you want strictly version 3.

Step 4: Verification Checklist

    Root Directory: Ensure LICENSE exists alongside Cargo.toml.

    README: Mention licensing in README.md:
    Markdown

    ## License
    Copyright (C) 2026 NATHANYEL WOOD. Licensed under the GNU General Public License v3.0 or later (GPL-3.0-or-later). See `LICENSE` for details.

    Crate Compatibility: If importing audio crates via crates.io (e.g., hound for WAV export, cpal or rodio for real-time audio playback), their licenses (typically MIT / Apache-2.0) are fully compatible with your outbound GPLv3 project.
