#!/usr/bin/env python3
# Copyright (C) 2026 NATHANYEL WOOD <nathanyelw@gmail.com>
# Licensed under the GNU General Public License v3.0 or later (GPL-3.0-or-later).
#
# RASPBERRY PI SINGLE-SWITCH METAL LAUNCHER (BCM GPIO 17 -> GND)
# Connects a physical toggle/slide switch on GPIO 17 to the native Rust
# `implant-hacks-r-us --switch-mode` 5-minute (300s) de-escalation engine!

import os
import subprocess
import sys

GPIO_PIN = 17
GPIO_VALUE_PATH = f"/sys/class/gpio/gpio{GPIO_PIN}/value"

def setup_gpio():
    if not os.path.exists(GPIO_VALUE_PATH):
        try:
            with open("/sys/class/gpio/export", "w") as f:
                f.write(str(GPIO_PIN))
            with open(f"/sys/class/gpio/gpio{GPIO_PIN}/direction", "w") as f:
                f.write("in")
        except PermissionError:
            print("[Hooked on Phon-et-ics] Note: /sys/class/gpio not exported; using virtual switch state.")

if __name__ == "__main__":
    setup_gpio()
    bin_path = os.path.join(os.path.dirname(__file__), "..", "target", "release", "implant-hacks-r-us")
    if not os.path.exists(bin_path):
        bin_path = "implant-hacks-r-us"
    os.execvp(bin_path, [bin_path, "--switch-mode", GPIO_VALUE_PATH])
