# `implant-hacks-r-us` // Hooked on Phon--et--ics DSP Toolkit

> [!CAUTION]
> **UNTESTED CODE USE AT YOUR OWN RISK - I AM NOT RESPONSIBLE FOR YOUR EARDRUMS, OR ANYONE ELSE'S, YOUR FUTURE, YOUR PAST, YOUR PRESENT, OR YOUR FAMILIES PROFOUND DISAPPOINTMENT IN YOU - DON'T BE IGNORANT IF IT DOESN'T WORK FIGURE IT OUT YOURSELF I DON'T HAVE TIME**
>
> **I AM MEDICAL UNPROFESSIONAL - DO NOT TAKE MY ADVICE - DON'T ASSUME THIS IS A GOOD OR BAD IDEA - THIS WAS INSPIRED BY WANTING TO MAKE A PHONE CALL TO THE PRESIDENT IN 1970! IF YOU SEND ME A CEASE AND DESIST BY EMAIL I WILL KEEP IT AS TREASURE**
>
> See [DISCLAIMER.md](file:///var/home/nwwood/Projects/all-in-one-open-life-project/advanced-meditation/DISCLAIMER.md) before running, flashing, or listening to anything in this repository.

> *"Call 1-800-ABCDEFG! Sound out your consonant blends, digraphs, diphthongs, and Haas precedence lags!"*

Dual-tone auditory matrix, escalating **Randomness Delta ($\Delta R$)**, binaural **Haas Spatial Time-Dilation**, and psychoacoustic defense escalation/de-escalation studio targeting auditory implant signal-processing bottlenecks.

Designed to work **out of the box with zero dependency woodwork** across:
- **Android OS & Any Browser**: Single self-contained `dist/web/index.html` Web App + compiled Rust WebAssembly (`implant_hacks_r_us.wasm`) + offline `.wav` pack
- **PinePhone 64 & Raspberry Pi (ARM64)**: 100% statically linked `aarch64-unknown-linux-musl` single binary (`dist/pinephone64-rpi-arm64/implant-hacks-r-us-static`) — zero shared libraries required
- **Linux Desktop & Flatpak**: Native single binary (`dist/linux-x86_64/implant-hacks-r-us`) and Flatpak (`flatpak/org.nwwood.ImplantHacksRUs.yml`)
- **Bare-Metal Microcontrollers (Single-Switch Escalator + 5-Minute De-Escalation)**:
  - **Teensy 4.1 (`NXP i.MXRT1062`)**: [`firmware/teensy41_imxrt1062/teensy41_imxrt1062.ino`](file:///var/home/nwwood/Projects/all-in-one-open-life-project/advanced-meditation/firmware/teensy41_imxrt1062/teensy41_imxrt1062.ino)
  - **ESP32**: [`firmware/esp32_escalator/esp32_escalator.ino`](file:///var/home/nwwood/Projects/all-in-one-open-life-project/advanced-meditation/firmware/esp32_escalator/esp32_escalator.ino)
  - **Raspberry Pi GPIO**: [`firmware/rpi_gpio_daemon.py`](file:///var/home/nwwood/Projects/all-in-one-open-life-project/advanced-meditation/firmware/rpi_gpio_daemon.py) or `implant-hacks-r-us --switch-mode`

---

## Features & Defense Architecture

1. **6-Tier Defense Escalation Ladder (Tiers 0–5) with Randomness Delta ($\Delta R$) & Spatial Time Dilation**:
   - **Tier 0 (Silent 'E' Grounding, $\Delta R = 0.00$)**: Pure `300/600/900 Hz` biological harmonics breathing at `4 Hz`.
   - **Tier 1 (Yellow Workbook — Crossover Smear, $\Delta R = 0.05$)**: Predictable `1050 + 1150 Hz` filterbank boundary contention.
   - **Tier 2 (Orange Workbook — Digraph Lock, $\Delta R = 0.35$)**: `85 Hz` basal envelope trap (`3450 + 3535 Hz`), `900 pps` CIS pulse train, and non-sequential tonotopic register hops.
   - **Tier 3 (Red Workbook — Phreak & AGC Clamp, $\Delta R = 0.68$)**: Randomizes `2700–5500 Hz` ear-canal resonance pairs to trip front-end Automatic Gain Control (AGC) + F1/F2 DTMF twist bursts with elastic silence gaps.
   - **Tier 4 (Purple Master Tape — Haas Anticipation Overdrive, $\Delta R = 0.95$)**: Randomizes electrode registers across all matrices while stretching/compressing inter-snap intervals and flipping `1.6–4.4 ms` Haas precedence lags Left/Right to keep the auditory cortex locked in continuous anticipation.
   - **Tier 5 (★ Secret Black Workbook — Full-Rail 220 Hz + 7200 Hz Overwrite + Grey Noise Static, $\Delta R = 1.00$)**: Drives continuous, un-gated `220 Hz` (apical bottom rail) and `7200 Hz` (basal top rail) tones at 100% duty cycle to overwrite the top and bottom of the implant/psychoacoustic range at all times, layered with inverse-Fletcher-Munson equal-loudness **Grey Noise static** on top of the active Tiers 0–4 background.
2. **4 Interactive 4x4 Keypads (Including Military AUTOVON `A, B, C, D` & Sub-100 Hz Ghost Pad)**:
   - `DTMF_TABLE` (`dmtf_phone_phreaker.py` — `697–1633 Hz` + 2600 Hz Blue Box trunk seize)
   - `IMPLANT_DTMF` (`synth_generator.py` — `220–7200 Hz` basilar map with Fletcher-Munson compensation)
   - `EAR_RESONANCE_DTMF` (`ear_resonance_generator.py` — `2700–5500 Hz` quarter-wave meatus cavity)
   - `SUB_100_GHOST_PAD` (*"Hot Cross Buns"* `12–99 Hz` sub-filterbank roll-off pad + **Tartini Missing-Fundamental Phantom Mode**)
3. **Single-Switch Bare-Metal Mode (5-Minute / 300s De-Escalation)**:
   - On ESP32, Teensy 4.1 (`i.MXRT1062`), and Raspberry Pi GPIO, a single toggle/slide switch controls the entire synthesizer: **Switch ON** slides up the escalation ladder; **Switch OFF** smoothly de-escalates over **5 minutes (`300.0 seconds`)** down to silence.

---

## Quick Start

```bash
# Build all targets (Linux x86_64, Static PinePhone 64 / RPi ARM64, WASM Web App, and 14 WAVs):
./build-all.sh

# Launch the interactive Web/Desktop Studio:
./dist/linux-x86_64/implant-hacks-r-us

# Export all 14 WAV files directly from the CLI:
./dist/linux-x86_64/implant-hacks-r-us --export-all ./my-wavs

# Run in Raspberry Pi / PinePhone 64 Single-Switch Hardware Mode (5-minute de-escalation):
./dist/pinephone64-rpi-arm64/implant-hacks-r-us-static --switch-mode /sys/class/gpio/gpio17/value
```

---

## License
Copyright (C) 2026 NATHANYEL WOOD. Licensed under the GNU General Public License v3.0 or later (GPL-3.0-or-later). See `LICENSE` for details.
