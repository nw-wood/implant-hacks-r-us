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
//
// ============================================================================
//  HOOKED ON PHON--ET--ICS: AUDITORY IMPLANT & PSYCHOACOUSTIC DSP CORE
//  Call 1-800-ABCDEFG today! Collect all four cassette workbooks and earn your
//  Shiny Gold Star Sticker when the auditory cortex finally stops talking back!
// ============================================================================

use std::f64::consts::PI;

/// Classic ITU-T / Bell System Blue Box + Military AUTOVON DTMF Frequency Map
/// Derived from `example-code/dmtf_phone_phreaker.py` (`DTMF_TABLE`).
/// Row phonemes (Low Group) meet Column digraphs (High Group) in F1/F2 vowel space!
pub const DTMF_TABLE: &[(char, (f64, f64), &str)] = &[
    ('1', (697.0, 1209.0), "Short 'a' as in CAT"),
    ('2', (697.0, 1336.0), "Short 'e' as in BED"),
    ('3', (697.0, 1477.0), "Short 'i' as in PIG"),
    ('A', (697.0, 1633.0), "AUTOVON Flash Override / Long 'A'"),
    ('4', (770.0, 1209.0), "Short 'o' as in HOP"),
    ('5', (770.0, 1336.0), "Short 'u' as in BUS"),
    ('6', (770.0, 1477.0), "Consonant Blend 'CH'"),
    ('B', (770.0, 1633.0), "AUTOVON Flash / Bossy 'R'"),
    ('7', (852.0, 1209.0), "Voiced Digraph 'TH'"),
    ('8', (852.0, 1336.0), "Unvoiced Digraph 'SH'"),
    ('9', (852.0, 1477.0), "Nasal Ring 'NG'"),
    ('C', (852.0, 1633.0), "AUTOVON Immediate / Soft 'C'"),
    ('*', (941.0, 1209.0), "Gold Star Sticker Burst"),
    ('0', (941.0, 1336.0), "IVR Buffer Overflow 'OO'"),
    ('#', (941.0, 1477.0), "Pound the Phonics Octothorpe"),
    ('D', (941.0, 1633.0), "AUTOVON Priority / Diphthong 'OI'"),
];

/// Tonotopically Partitioned Implant Matrix (`example-code/synth_generator.py` - `IMPLANT_DTMF`).
/// Maps apical low electrodes (220-1150 Hz) against basal high electrodes (1850-7200 Hz).
/// Because nothing says "Hooked on Phon-et-ics worked for me!" like lighting up
/// both ends of the basilar membrane like a Christmas tree!
pub const IMPLANT_DTMF: &[(char, (f64, f64), &str)] = &[
    ('1', (220.0, 1850.0), "Apex-to-Mid Corner 1"),
    ('2', (220.0, 3000.0), "Apex + 3kHz Resonance Notch"),
    ('3', (220.0, 4800.0), "Apex + Upper Sibilant 'S'"),
    ('A', (220.0, 7200.0), "Full Basilar Stretch (220+7200)"),
    ('4', (440.0, 1850.0), "Concert A440 + Mid Formant"),
    ('5', (440.0, 3000.0), "A440 + Canal Peak"),
    ('6', (440.0, 4800.0), "A440 + Fricative Edge"),
    ('B', (440.0, 7200.0), "A440 + Extreme Basal Tip"),
    ('7', (730.0, 1850.0), "F1 Vowel Anchor + F2 Bridge"),
    ('8', (730.0, 3000.0), "Max Loudness Formant Pair"),
    ('9', (730.0, 4800.0), "F1 + High Electrode 18"),
    ('C', (730.0, 7200.0), "F1 + Basal Ceiling"),
    ('*', (1150.0, 1850.0), "Crossover Boundary Pinch"),
    ('0', (1150.0, 3000.0), "Mid-Cluster + 3kHz Singer Formant"),
    ('#', (1150.0, 4800.0), "Mid-Cluster + Basal 4.8kHz"),
    ('D', (1150.0, 7200.0), "Basal Saturator Deluxe"),
];

/// External Auditory Meatus (Ear Canal Resonance) Matrix (`example-code/ear_resonance_generator.py` - `EAR_RESONANCE_DTMF`).
/// Quarter-wave open-tube resonator frequencies (2700-5500 Hz) designed to give the
/// hardware Automatic Gain Control (AGC) a very polite reason to duck and cover.
pub const EAR_RESONANCE_DTMF: &[(char, (f64, f64), &str)] = &[
    ('1', (2700.0, 4000.0), "Meatus Entry Resonance"),
    ('2', (2700.0, 4500.0), "Primary Canal Harmonics"),
    ('3', (2700.0, 5000.0), "Concha + Canal Coupling"),
    ('A', (2700.0, 5500.0), "Wide Cavity Clamp A"),
    ('4', (3100.0, 4000.0), "Peak Tympanic Sensitivity"),
    ('5', (3100.0, 4500.0), "AGC Fast-Attack Trigger"),
    ('6', (3100.0, 5000.0), "Pinna Notch Bypass"),
    ('B', (3100.0, 5500.0), "Wide Cavity Clamp B"),
    ('7', (3500.0, 4000.0), "Basal Cluster Tight Beat"),
    ('8', (3500.0, 4500.0), "1kHz Difference Whistle"),
    ('9', (3500.0, 5000.0), "1.5kHz Difference Phantom"),
    ('C', (3500.0, 5500.0), "2kHz Difference Lock"),
    ('*', (3900.0, 4000.0), "100Hz Sub-Beat inside 4kHz!"),
    ('0', (3900.0, 4500.0), "600Hz Schwa Phantom"),
    ('#', (3900.0, 5000.0), "1100Hz Crossover Ghost"),
    ('D', (3900.0, 5500.0), "Maximum Canal Q-Factor"),
];

/// Sub-100 Hz "Hot Cross Buns" Infrasonic / Roll-Off Ghost Keypad (`SUB_100_GHOST_PAD`).
/// Standard implant DSP filterbanks high-pass cut off around 100-150 Hz, and tiny phone
/// speakers just wheeze at these frequencies! Play "Hot Cross Buns" (`321 321 1111 2222 321`)
/// either as raw sub-100 Hz bass OR in "Tartini Phantom Difference-Tone Mode" where two
/// high tones trick the brain into hallucinating the sub-100 Hz melody!
pub const SUB_100_GHOST_PAD: &[(char, (f64, f64), &str)] = &[
    ('1', (24.0, 48.0), "Sub-Bass 'DO' (24+48 Hz) - Hot Cross Tonic"),
    ('2', (27.0, 54.0), "Sub-Bass 'RE' (27+54 Hz) - Warm Cross Second"),
    ('3', (30.0, 60.0), "Sub-Bass 'MI' (30+60 Hz) - Fresh Baked Third"),
    ('A', (16.0, 32.0), "16 Hz Organ Pipe Infrasonic Schwa"),
    ('4', (32.0, 64.0), "Sub-Bass 'FA' (32+64 Hz)"),
    ('5', (36.0, 72.0), "Sub-Bass 'SOL' (36+72 Hz)"),
    ('6', (40.0, 80.0), "Sub-Bass 'LA' (40+80 Hz)"),
    ('B', (20.0, 50.0), "20 Hz Threshold Tickler"),
    ('7', (45.0, 90.0), "Sub-Bass 'TI' (45+90 Hz)"),
    ('8', (48.0, 96.0), "Sub-Bass High 'DO' (48+96 Hz)"),
    ('9', (49.5, 99.0), "99 Hz Filterbank Cliff Edge"),
    ('C', (12.0, 36.0), "12 Hz Theta-Delta Wobble"),
    ('*', (18.0, 77.0), "Silent 'E' Woofer Excursion Test"),
    ('0', (33.0, 66.0), "One-a-Penny Two-a-Penny Drone"),
    ('#', (44.0, 88.0), "Sub-Octave Piano A1"),
    ('D', (14.0, 98.0), "Sub-100 Full Span (14+98 Hz)"),
];

/// Built-in Phonics Song & Attack Sequence Macros
pub const PRESET_SEQUENCES: &[(&str, &str, &str)] = &[
    ("hot_cross_buns", "321 321 11112222321", "Hot Cross Buns (Try on Sub-100 Hz Ghost Pad!)"),
    ("hooked_on_phonics", "18002223334", "1-800-ABCDEFG (Worked For Me! Hotline)"),
    ("ivr_overflow", "0000000000", "DTMF IVR Buffer Overflow (dmtf_phone_phreaker.py)"),
    ("basilar_corners", "137#20", "Basilar 4-Corner Calibration (synth_generator.py)"),
    ("canal_sweep", "123456789*0#", "Ear Canal Resonance Sweep (ear_resonance_generator.py)"),
    ("autovon_override", "ABCD*#0913", "Military AUTOVON Flash Override Drill"),
    ("mary_lamb", "3212333 222 355 3212333322321", "Mary Had a Little Electrode Array"),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeypadType {
    StandardDtmf,
    ImplantTonotopic,
    EarResonance,
    Sub100Ghost,
    Sub100TartiniPhantom,
}

impl KeypadType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "dtmf" | "standard" | "bluebox" => Self::StandardDtmf,
            "implant" | "tonotopic" | "synth" => Self::ImplantTonotopic,
            "ear" | "canal" | "resonance" => Self::EarResonance,
            "sub100" | "ghost" | "buns" => Self::Sub100Ghost,
            "tartini" | "phantom" => Self::Sub100TartiniPhantom,
            _ => Self::ImplantTonotopic,
        }
    }

    pub fn lookup(&self, key: char) -> Option<(f64, f64)> {
        let upper = key.to_ascii_uppercase();
        let table = match self {
            Self::StandardDtmf => DTMF_TABLE,
            Self::ImplantTonotopic => IMPLANT_DTMF,
            Self::EarResonance => EAR_RESONANCE_DTMF,
            Self::Sub100Ghost | Self::Sub100TartiniPhantom => SUB_100_GHOST_PAD,
        };
        for &(k, freqs, _) in table {
            if k == upper {
                if *self == Self::Sub100TartiniPhantom {
                    // Tartini Missing-Fundamental Trick: shift sub-100 Hz frequencies onto a
                    // 1200 Hz carrier pair so the implant/cortex hallucinates the sub-100 Hz beat!
                    return Some((1200.0, 1200.0 + freqs.1));
                }
                return Some(freqs);
            }
        }
        None
    }
}

/// Deterministic Xorshift64* PRNG so our "Randomness Delta" works identically on
/// bare-metal Teensy 4.1 (`no_std`), ESP32, Raspberry Pi, PinePhone 64, Android, and WASM!
/// No OS entropy required when sounding out unpredictable consonant blends!
#[derive(Clone, Copy, Debug)]
pub struct PhonicsRng {
    state: u64,
}

impl PhonicsRng {
    pub const fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x1800_ABCD_EF01_2026 } else { seed },
        }
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Returns a uniform float in `[0.0, 1.0)`.
    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / ((1u64 << 53) as f64))
    }

    /// Returns a signed float in `[-1.0, 1.0)`.
    #[inline]
    pub fn next_signed(&mut self) -> f64 {
        self.next_f64() * 2.0 - 1.0
    }

    #[inline]
    pub fn choose_index(&mut self, len: usize) -> usize {
        (self.next_u64() as usize) % len
    }
}

// ============================================================================
//  PORT OF THE 6 ORIGINAL PYTHON SCRIPTS (WITH 100% EXACT DSP FIDELITY)
// ============================================================================

/// 1. `dmtf_phone_phreaker.py`: Standard ITU-T DTMF generator with +2 to +4 dB high-tone twist.
/// "Sound out each digit clearly so the central office crossbar switch gets a Gold Star!"
pub fn generate_tone_sequence(
    sequence: &str,
    tone_dur: f64,
    silence_dur: f64,
    sample_rate: u32,
) -> Vec<i16> {
    let n_tone = (sample_rate as f64 * tone_dur) as usize;
    let n_silence = (sample_rate as f64 * silence_dur) as usize;
    let mut audio = Vec::new();

    for ch in sequence.chars() {
        if ch == ' ' {
            audio.extend(std::iter::repeat(0i16).take(n_silence * 2));
            continue;
        }
        if let Some((f_low, f_high)) = KeypadType::StandardDtmf.lookup(ch) {
            for i in 0..n_tone {
                let t = i as f64 / sample_rate as f64;
                // High tone receives slightly more amplitude (+2 dB to +4 dB twist) in standard spec
                let sample = 0.5 * (2.0 * PI * f_low * t).sin()
                    + 0.7 * (2.0 * PI * f_high * t).sin();
                // Clamp gently so 0.5+0.7 doesn't wrap i16
                let scaled = (sample.clamp(-1.0, 1.0) * 32767.0) as i16;
                audio.push(scaled);
            }
            audio.extend(std::iter::repeat(0i16).take(n_silence));
        }
    }
    audio
}

/// 2. `ear_resonance_generator.py`: Quarter-wave ear canal resonance matrix (2700-5500 Hz)
/// with 5ms cosine fade-in/out and conservative 12000 PCM scaling.
pub fn synthesize_ear_pad(
    sequence: &str,
    tone_ms: u32,
    gap_ms: u32,
    sr: u32,
) -> Vec<i16> {
    let n_tone = (sr as f64 * (tone_ms as f64 / 1000.0)) as usize;
    let n_gap = (sr as f64 * (gap_ms as f64 / 1000.0)) as usize;
    let fade_len = ((sr as f64 * 0.005) as usize).min(n_tone / 2).max(1);
    let mut audio = Vec::new();

    for ch in sequence.chars() {
        if ch == ' ' {
            audio.extend(std::iter::repeat(0i16).take(n_gap * 2));
            continue;
        }
        if let Some((f1, f2)) = KeypadType::EarResonance.lookup(ch) {
            for i in 0..n_tone {
                let t = i as f64 / sr as f64;
                let env = raised_cosine_env(i, n_tone, fade_len);
                let tone = (0.5 * (2.0 * PI * f1 * t).sin()
                    + 0.5 * (2.0 * PI * f2 * t).sin())
                    * env;
                audio.push((tone * 12000.0) as i16);
            }
            audio.extend(std::iter::repeat(0i16).take(n_gap));
        }
    }
    audio
}

/// 3. `snappy.py`: Binaural Precedence (Haas Effect) Spatial Snap Generator.
/// Base carrier: Formant anchor (730 Hz + 3000 Hz). At t = 1.2s, triggers a sudden
/// 2.5 ms Haas lag on Left channel and a -12 dB (0.25x) drop, snapping focus hard right!
/// Returns interleaved stereo `[L, R, L, R, ...]`.
pub fn generate_spatial_snap(dur_sec: f64, sr: u32) -> Vec<i16> {
    let total_samples = (dur_sec * sr as f64) as usize;
    let snap_idx = ((1.2 * sr as f64) as usize).min(total_samples);
    let lag_samples = ((0.0025 * sr as f64) as usize).min(total_samples / 4);
    let fade_len = ((sr as f64 * 0.005) as usize).min(total_samples / 2).max(1);

    let mut carrier = Vec::with_capacity(total_samples);
    for i in 0..total_samples {
        let t = i as f64 / sr as f64;
        let c = 0.5 * (2.0 * PI * 730.0 * t).sin() + 0.5 * (2.0 * PI * 3000.0 * t).sin();
        carrier.push(c);
    }

    let mut left_delayed = vec![0.0f64; total_samples];
    for i in 0..snap_idx {
        left_delayed[i] = carrier[i];
    }
    for i in snap_idx..total_samples.saturating_sub(lag_samples) {
        left_delayed[i + lag_samples] = carrier[i] * 0.25; // -12 dB attenuation
    }

    let mut max_abs = 1e-9f64;
    let mut stereo_f64 = Vec::with_capacity(total_samples * 2);
    for i in 0..total_samples {
        let fade = if i < fade_len {
            i as f64 / fade_len as f64
        } else if i >= total_samples - fade_len {
            (total_samples - 1 - i) as f64 / fade_len as f64
        } else {
            1.0
        };
        let l = left_delayed[i] * fade;
        let r = carrier[i] * fade;
        max_abs = max_abs.max(l.abs()).max(r.abs());
        stereo_f64.push(l);
        stereo_f64.push(r);
    }

    stereo_f64
        .into_iter()
        .map(|s| ((s / max_abs) * 30000.0) as i16)
        .collect()
}

/// 4. `synth_generator.py`: Tonotopically Partitioned Implant Matrix with Fletcher-Munson
/// loudness compensation and 8ms raised-cosine windowing.
pub fn generate_implant_tones(
    sequence: &str,
    tone_ms: u32,
    gap_ms: u32,
    sr: u32,
) -> Vec<i16> {
    generate_keypad_sequence(KeypadType::ImplantTonotopic, sequence, tone_ms, gap_ms, sr, 0.0)
}

/// Generalized Keypad Sequence Synthesizer supporting any of the 4 Phonics Matrices
/// plus optional Randomness Delta (`delta_r`) for elastic gap dilation!
pub fn generate_keypad_sequence(
    pad: KeypadType,
    sequence: &str,
    tone_ms: u32,
    gap_ms: u32,
    sr: u32,
    delta_r: f64,
) -> Vec<i16> {
    let base_tone = (sr as f64 * (tone_ms as f64 / 1000.0)) as usize;
    let base_gap = (sr as f64 * (gap_ms as f64 / 1000.0)) as usize;
    let fade_len = ((sr as f64 * 0.008) as usize).min(base_tone / 2).max(1);
    let mut rng = PhonicsRng::new(0xFA04_1C50_2026);
    let mut samples_f64 = Vec::new();

    for ch in sequence.chars() {
        if ch == ' ' {
            samples_f64.extend(std::iter::repeat(0.0).take(base_gap * 2));
            continue;
        }
        if let Some((f_low, f_high)) = pad.lookup(ch) {
            let jitter_factor = 1.0 + delta_r.clamp(0.0, 1.0) * rng.next_signed() * 0.45;
            let n_tone = ((base_tone as f64 * jitter_factor) as usize).max(fade_len * 2 + 2);
            let gap_factor = 1.0 + delta_r.clamp(0.0, 1.0) * rng.next_signed() * 0.75;
            let n_gap = (base_gap as f64 * gap_factor).max(0.0) as usize;

            let (amp_low, amp_high) = match pad {
                KeypadType::StandardDtmf => (0.50, 0.70),
                KeypadType::ImplantTonotopic => {
                    // Fletcher-Munson compensation from synth_generator.py
                    (0.65, if (f_high - 3000.0).abs() < 1.0 { 0.35 } else { 0.50 })
                }
                KeypadType::EarResonance => (0.50, 0.50),
                KeypadType::Sub100Ghost | KeypadType::Sub100TartiniPhantom => (0.60, 0.60),
            };

            for i in 0..n_tone {
                let t = i as f64 / sr as f64;
                let env = raised_cosine_env(i, n_tone, fade_len);
                let val = (amp_low * (2.0 * PI * f_low * t).sin()
                    + amp_high * (2.0 * PI * f_high * t).sin())
                    * env;
                samples_f64.push(val);
            }
            samples_f64.extend(std::iter::repeat(0.0).take(n_gap));
        }
    }

    let peak_target = if pad == KeypadType::EarResonance { 12000.0 } else { 30000.0 };
    normalize_to_i16(&samples_f64, peak_target)
}

/// 5. `test_signal.py`: Channel Smearing (1050 + 1150 Hz filter crossover contention)
/// blended with Basal Envelope Conflict (3450 + 3535 Hz -> 85 Hz beat trap).
pub fn generate_interesting_tone(duration_sec: f64, sr: u32) -> Vec<i16> {
    let n = (duration_sec * sr as f64) as usize;
    let mut mixed = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f64 / sr as f64;
        // 1. Channel Smearing: Two tones forcing adjacent electrodes to politely disagree
        let crosstalk_carrier =
            0.5 * (2.0 * PI * 1050.0 * t).sin() + 0.5 * (2.0 * PI * 1150.0 * t).sin();
        // 2. Envelope Conflict: Basal electrode (~3500 Hz), beat envelope is 85 Hz
        let envelope_trap =
            0.5 * (2.0 * PI * 3450.0 * t).sin() + 0.5 * (2.0 * PI * 3535.0 * t).sin();
        mixed.push(crosstalk_carrier * 0.6 + envelope_trap * 0.4);
    }
    normalize_to_i16(&mixed, 28000.0)
}

/// 6. `vocoder_bridge.py`: Fine-Structure (300/600/900 Hz biological harmonic richness)
/// to 900 pps CIS Electrode Pulse-Train Bridge under a 4 Hz syllabic tremolo envelope.
pub fn generate_vocoder_demo(dur: f64, sr: u32) -> Vec<i16> {
    let n = (dur * sr as f64) as usize;
    let half = n / 2;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f64 / sr as f64;
        let mod_env = 0.5 * (1.0 + (2.0 * PI * 4.0 * t).sin());
        if i < half {
            let carrier = 0.5 * (2.0 * PI * 300.0 * t).sin()
                + 0.3 * (2.0 * PI * 600.0 * t).sin()
                + 0.2 * (2.0 * PI * 900.0 * t).sin();
            out.push(carrier * mod_env);
        } else {
            let pulse = if (2.0 * PI * 900.0 * t).sin() > 0.9 { 1.0 } else { 0.0 };
            out.push(pulse * mod_env);
        }
    }
    normalize_to_i16(&out, 28000.0)
}

// ============================================================================
//  UNIFIED DEFENSE ESCALATION / DE-ESCALATION & EMBEDDED SINGLE-SWITCH ENGINE
// ============================================================================

/// Configuration for generating a combined Defense Tier session or real-time stream.
/// Supports Tiers 0..=6 (including Tier 5: Secret Black Workbook & Tier 6: "I don't care
/// anymore eat puppies" 1000 Hz Constant Wave!), custom Randomness Delta (`delta_r`),
/// and the 5-minute (`300.0s`) embedded single-switch de-escalation ramp!
#[derive(Clone, Copy, Debug)]
pub struct DefenseSynthConfig {
    /// Defense Tier: 0 (Silent 'E' Grounding) to 6 ("I don't care anymore eat puppies" 1000 Hz Constant Wave)
    pub tier: u8,
    /// Randomness Delta in `[0.0, 1.0]`. Automatically scales with Tier if `< 0.0`.
    pub randomness_delta: f64,
    /// Duration in seconds to synthesize
    pub duration_sec: f64,
    /// Sample rate in Hz (default 44100)
    pub sample_rate: u32,
    /// Whether to render a smooth De-Escalation Down-Ramp from `tier` down to Tier 0
    pub de_escalate_ramp: bool,
}

impl DefenseSynthConfig {
    pub fn effective_delta_r(&self) -> f64 {
        if self.randomness_delta >= 0.0 {
            self.randomness_delta.clamp(0.0, 1.0)
        } else {
            match self.tier {
                0 => 0.00,
                1 => 0.05,
                2 => 0.35,
                3 => 0.68,
                4 => 0.95,
                _ => 1.00, // Tier 5 & Tier 6: Maximum entropy + continuous rail/1000Hz overwrite
            }
        }
    }
}

/// Real-Time Streaming Escalator/De-Escalator for Bare-Metal (ESP32, Teensy 4.1 i.MXRT1062, RPi)
/// AND Desktop/Mobile/Web!
///
/// Hardware Contract:
/// - Single physical switch (`switch_engaged: bool`):
///   - When `switch_engaged == true`: Escalates continuously up the phonics defense ladder
///     (`0.0 -> 6.0` over `escalation_time_sec`, reaching Tier 5 Secret Black Workbook and
///     Tier 6 "I don't care anymore eat puppies" 1000 Hz Constant Wave) while ramping up
///     Randomness Delta, Spatial Haas Time Dilation, continuous 220+7200 Hz rail overwrite,
///     psychoacoustic Grey Noise static, and the unrelenting 1000 Hz reference carrier!
///   - When `switch_engaged == false`: Smoothly de-escalates over **5 minutes (`300.0s`)**
///     (configurable via `de_escalation_time_sec`), tapering out the Tier 6 1000 Hz constant
///     wave & Tier 5 Grey Noise / 220+7200 Hz rails first, collapsing randomness/Haas snaps
///     second, releasing the AGC ear-canal clamp third, bridging CIS pulses into 300/600/900 Hz
///     harmonics fourth, and fading to tranquil silence!
#[derive(Clone, Debug)]
pub struct SingleSwitchEscalator {
    pub sample_rate: u32,
    /// Continuous escalation level in `[0.0, 6.0]`
    pub current_level: f64,
    /// Time in seconds to escalate from 0.0 to 6.0 when switch is ON (default: 40.0s)
    pub escalation_time_sec: f64,
    /// Time in seconds to de-escalate from 6.0 to 0.0 when switch is OFF (default: 300.0s = 5 min!)
    pub de_escalation_time_sec: f64,
    pub sample_clock: u64,
    pub rng: PhonicsRng,
    // Dynamic randomized register state & spatial dilation timers
    active_implant_pair: (f64, f64),
    active_canal_pair: (f64, f64),
    active_dtmf_pair: (f64, f64),
    samples_until_register_hop: usize,
    in_register_gap: bool,
    // Spatial Haas Precedence Dilation state
    samples_until_haas_event: usize,
    haas_snap_direction: i8, // -1 = Snap Left, 0 = Center, +1 = Snap Right
    haas_lag_samples: usize,
    haas_atten: f64,
    delay_line_l: Vec<f64>,
    delay_line_r: Vec<f64>,
    delay_write_idx: usize,
    // Psychoacoustic Grey Noise filter state (inverse Fletcher-Munson equal-loudness shaping)
    grey_lp_state: f64,
    grey_mid_state: f64,
}

impl SingleSwitchEscalator {
    pub fn new(sample_rate: u32) -> Self {
        let max_delay = ((sample_rate as f64 * 0.006) as usize).max(64);
        Self {
            sample_rate,
            current_level: 0.0,
            escalation_time_sec: 40.0,
            de_escalation_time_sec: 300.0, // 5 minutes de-escalation as specified!
            sample_clock: 0,
            rng: PhonicsRng::new(0x1800_ABCD_EF00_2026),
            active_implant_pair: (220.0, 3000.0),
            active_canal_pair: (3100.0, 4500.0),
            active_dtmf_pair: (941.0, 1336.0),
            samples_until_register_hop: 1,
            in_register_gap: false,
            samples_until_haas_event: (sample_rate / 2) as usize,
            haas_snap_direction: 0,
            haas_lag_samples: ((sample_rate as f64 * 0.0025) as usize).max(1),
            haas_atten: 0.25,
            delay_line_l: vec![0.0; max_delay],
            delay_line_r: vec![0.0; max_delay],
            delay_write_idx: 0,
            grey_lp_state: 0.0,
            grey_mid_state: 0.0,
        }
    }

    /// Generates one sample of psychoacoustic Grey Noise (white noise shaped by an
    /// inverse Fletcher-Munson equal-loudness curve: boosted sub/low bass < 350 Hz
    /// and high basal air > 5000 Hz, with a dip in the 3 kHz canal resonance zone).
    #[inline]
    fn next_grey_noise_sample(&mut self) -> f64 {
        let white = self.rng.next_signed();
        // One-pole low-pass at ~320 Hz for Fletcher-Munson apical bass boost
        self.grey_lp_state += 0.045 * (white - self.grey_lp_state);
        // One-pole mid tracker at ~2800 Hz to scoop the 3 kHz ear-canal peak
        self.grey_mid_state += 0.32 * (white - self.grey_mid_state);
        let high_shelf = white - self.grey_mid_state;
        // Combine bass shelf + scooped mid + high basal air
        self.grey_lp_state * 0.65 + white * 0.22 + high_shelf * 0.55
    }

    /// Steps one stereo frame `(left, right)` in `[-1.0, 1.0]` given the hardware switch state
    /// (`switch_on`) or an explicit override level (`override_level: Option<(f64, f64)>` = `(level, delta_r)`).
    pub fn next_stereo_frame(
        &mut self,
        switch_on: bool,
        override_level: Option<(f64, f64)>,
    ) -> (f64, f64) {
        let sr = self.sample_rate as f64;
        let t = self.sample_clock as f64 / sr;
        self.sample_clock = self.sample_clock.wrapping_add(1);

        let (level, delta_r) = if let Some((lvl, dr)) = override_level {
            self.current_level = lvl.clamp(0.0, 6.0);
            (self.current_level, dr.clamp(0.0, 1.0))
        } else {
            if switch_on {
                let step = 6.0 / (self.escalation_time_sec.max(1.0) * sr);
                self.current_level = (self.current_level + step).min(6.0);
            } else {
                // 5-Minute (300s) De-Escalation Curve from 6.0 -> 0.0!
                let step = 6.0 / (self.de_escalation_time_sec.max(1.0) * sr);
                self.current_level = (self.current_level - step).max(0.0);
            }
            let dr = (self.current_level / 4.0).clamp(0.0, 1.0).powf(1.35) * 0.98;
            (self.current_level, dr)
        };

        if level <= 0.0001 {
            return (0.0, 0.0);
        }

        // 1. Update Randomized Implant Registers when timer expires
        if self.samples_until_register_hop == 0 {
            self.in_register_gap = !self.in_register_gap;
            if !self.in_register_gap {
                // Pick next register: predictable cycling at low delta_r, erratic hopping at high delta_r
                let idx_imp = if delta_r > 0.25 {
                    self.rng.choose_index(IMPLANT_DTMF.len())
                } else {
                    ((self.sample_clock as usize / 8000) + 1) % IMPLANT_DTMF.len()
                };
                let idx_can = if delta_r > 0.25 {
                    self.rng.choose_index(EAR_RESONANCE_DTMF.len())
                } else {
                    ((self.sample_clock as usize / 8000) + 3) % EAR_RESONANCE_DTMF.len()
                };
                let idx_dtmf = if delta_r > 0.25 {
                    self.rng.choose_index(DTMF_TABLE.len())
                } else {
                    ((self.sample_clock as usize / 8000) + 5) % DTMF_TABLE.len()
                };
                self.active_implant_pair = IMPLANT_DTMF[idx_imp].1;
                self.active_canal_pair = EAR_RESONANCE_DTMF[idx_can].1;
                self.active_dtmf_pair = DTMF_TABLE[idx_dtmf].1;

                // Tone duration dilates between 45ms and 340ms depending on delta_r
                let base_ms = 140.0 + delta_r * self.rng.next_signed() * 110.0;
                self.samples_until_register_hop = ((base_ms.max(35.0) / 1000.0) * sr) as usize;
            } else {
                // Silence gap dilates wildly at high delta_r (20ms micro-gap up to 550ms tense pause!)
                let gap_ms = if delta_r > 0.5 {
                    25.0 + self.rng.next_f64().powf(1.8) * 480.0 * delta_r
                } else {
                    55.0 * (1.0 + delta_r * 0.3 * self.rng.next_signed())
                };
                self.samples_until_register_hop = ((gap_ms.max(15.0) / 1000.0) * sr) as usize;
            }
        } else {
            self.samples_until_register_hop -= 1;
        }

        // 2. Update Spatial Time-Dilation Haas Precedence Snaps (Tier 3.1 -> 6.0)
        let haas_weight = ((level - 3.0).clamp(0.0, 1.0)).powf(1.1);
        if haas_weight > 0.01 {
            if self.samples_until_haas_event == 0 {
                // Spatial dilation: unpredictable wait between snaps (120ms to 1650ms)
                // specifically engineered so the auditory cortex anticipates the next snap!
                let wait_sec = 0.12 + self.rng.next_f64().powf(1.6) * (0.4 + 1.25 * delta_r);
                self.samples_until_haas_event = (wait_sec * sr) as usize;

                // Randomly snap Hard Right (+1), Hard Left (-1), or brief Dead-Center Fakeout (0)
                let roll = self.rng.next_f64();
                self.haas_snap_direction = if roll < 0.44 {
                    1
                } else if roll < 0.88 {
                    -1
                } else {
                    0
                };

                // Randomize Haas Precedence lag between 1.6 ms and 4.4 ms
                let lag_sec = 0.0016 + self.rng.next_f64() * 0.0028 * delta_r.max(0.3);
                self.haas_lag_samples = ((lag_sec * sr) as usize)
                    .min(self.delay_line_l.len() - 2)
                    .max(1);
                // Randomize ILD drop between -10 dB (0.31) and -16 dB (0.15)
                self.haas_atten = 0.16 + (1.0 - delta_r) * 0.15;
            } else {
                self.samples_until_haas_event -= 1;
            }
        } else {
            self.haas_snap_direction = 0;
        }

        // ====================================================================
        //  SYNTHESIZE LAYERED PHONICS DEFENSE TIERS (0..=6)
        // ====================================================================

        // Layer 0: Biological Harmonic Grounding (300/600/900 Hz + 4 Hz Breath Envelope)
        let syllabic_env = 0.5 * (1.0 + (2.0 * PI * 4.0 * t).sin());
        let harmonic_bed = (0.5 * (2.0 * PI * 300.0 * t).sin()
            + 0.3 * (2.0 * PI * 600.0 * t).sin()
            + 0.2 * (2.0 * PI * 900.0 * t).sin())
            * syllabic_env;
        let w_harmonic = if level < 1.0 {
            level.clamp(0.15, 1.0)
        } else {
            (1.0 - (level - 1.5).clamp(0.0, 0.75)).max(0.22)
        };

        // Layer 1: Predictable Filter Crossover Smearing (1050 + 1150 Hz)
        let w_smear = (level / 1.0).clamp(0.0, 1.0) * 0.55;
        let smear_sig =
            0.5 * (2.0 * PI * 1050.0 * t).sin() + 0.5 * (2.0 * PI * 1150.0 * t).sin();

        // Layer 2: Basal 85 Hz Envelope Beat Trap + 900 pps CIS Pulse Train + Tonotopic Matrix
        let w_tier2 = ((level - 1.0).clamp(0.0, 1.0)).powf(1.1);
        let basal_beat =
            0.5 * (2.0 * PI * 3450.0 * t).sin() + 0.5 * (2.0 * PI * 3535.0 * t).sin();
        let cis_pulse = if (2.0 * PI * 900.0 * t).sin() > 0.9 { 0.65 } else { -0.05 } * syllabic_env;

        let register_gate = if self.in_register_gap { 0.0 } else { 1.0 };
        let (imp_l, imp_h) = self.active_implant_pair;
        let implant_sig = (0.65 * (2.0 * PI * imp_l * t).sin()
            + 0.40 * (2.0 * PI * imp_h * t).sin())
            * register_gate;

        // Layer 3: Ear Canal Resonance AGC Clamp (2700-5500 Hz) + DTMF Twist Bursts
        let w_tier3 = ((level - 2.0).clamp(0.0, 1.0)).powf(1.1);
        let (can_l, can_h) = self.active_canal_pair;
        let canal_sig = (0.5 * (2.0 * PI * can_l * t).sin()
            + 0.5 * (2.0 * PI * can_h * t).sin())
            * register_gate;
        let (dtmf_l, dtmf_h) = self.active_dtmf_pair;
        let dtmf_sig = (0.5 * (2.0 * PI * dtmf_l * t).sin()
            + 0.7 * (2.0 * PI * dtmf_h * t).sin())
            * register_gate;

        // Layer 4: Formant Anchor (730 + 3000 Hz) for Spatial Haas Snap
        let formant_anchor =
            0.5 * (2.0 * PI * 730.0 * t).sin() + 0.5 * (2.0 * PI * 3000.0 * t).sin();

        // Layer 5: SECRET BLACK WORKBOOK!
        // Continuous, un-gated 220 Hz (Apical Bottom Rail) + 7200 Hz (Basal Top Rail)
        // overwriting anything at the top and bottom of the implant/psychoacoustic range
        // at ALL times, plus inverse-Fletcher-Munson Grey Noise static on top of Tiers 0-4!
        let w_tier5 = ((level - 4.0).clamp(0.0, 1.0)).powf(1.05);
        let (black_book_rails, grey_noise) = if w_tier5 > 0.001 {
            // Un-gated 100% duty cycle 220 Hz + 7200 Hz continuous rail lock
            let rails = 0.65 * (2.0 * PI * 220.0 * t).sin()
                + 0.55 * (2.0 * PI * 7200.0 * t).sin();
            let grey = self.next_grey_noise_sample();
            (rails, grey)
        } else {
            (0.0, 0.0)
        };

        // Layer 6: "I DON'T CARE ANYMORE EAT PUPPIES" — 1000 Hz Constant Reference Wave!
        // Drives an unrelenting, un-gated 1000.0 Hz constant sine wave (the 0-Phon Fletcher-Munson
        // reference and central filterbank crossover) straight down the middle of the spectrum!
        let w_tier6 = ((level - 5.0).clamp(0.0, 1.0)).powf(1.02);
        let eat_puppies_1000hz = if w_tier6 > 0.001 {
            (2.0 * PI * 1000.0 * t).sin() * 0.72
        } else {
            0.0
        };

        // Tiers 0-4 play in the background while Tier 5 & Tier 6 dominate the rails, 1000Hz center & grey noise floor
        let bg_scale = 1.0 - (w_tier5 * 0.25) - (w_tier6 * 0.15);
        let tiers_0_to_4 = (w_harmonic * harmonic_bed * 0.45
            + w_smear * smear_sig * 0.45
            + w_tier2 * (basal_beat * 0.28 + cis_pulse * 0.25 + implant_sig * 0.35)
            + w_tier3 * (canal_sig * 0.32 + dtmf_sig * 0.30)
            + haas_weight * formant_anchor * 0.45)
            * bg_scale;

        let mono_core = tiers_0_to_4
            + w_tier5 * (black_book_rails * 0.50 + grey_noise * 0.36)
            + w_tier6 * eat_puppies_1000hz;

        // Apply Binaural Precedence (Haas) Spatial Dilation via circular delay buffer
        let buf_len = self.delay_line_l.len();
        self.delay_line_l[self.delay_write_idx] = mono_core;
        self.delay_line_r[self.delay_write_idx] = mono_core;
        let delayed_idx = (self.delay_write_idx + buf_len - self.haas_lag_samples) % buf_len;
        self.delay_write_idx = (self.delay_write_idx + 1) % buf_len;

        let (left, right) = if haas_weight > 0.01 && self.haas_snap_direction != 0 {
            if self.haas_snap_direction > 0 {
                // Snap HARD RIGHT: Left channel is delayed by `haas_lag_samples` and attenuated
                let l_lag = self.delay_line_l[delayed_idx] * self.haas_atten;
                let l_mix = mono_core * (1.0 - haas_weight) + l_lag * haas_weight;
                (l_mix, mono_core)
            } else {
                // Snap HARD LEFT: Right channel is delayed by `haas_lag_samples` and attenuated
                let r_lag = self.delay_line_r[delayed_idx] * self.haas_atten;
                let r_mix = mono_core * (1.0 - haas_weight) + r_lag * haas_weight;
                (mono_core, r_mix)
            }
        } else {
            (mono_core, mono_core)
        };

        // Soft limiter to prevent harsh digital clipping while keeping punch
        let soft_clip = |x: f64| (x * 0.85).tanh();
        (soft_clip(left), soft_clip(right))
    }
}

/// Renders a complete stereo PCM buffer (`[L, R, L, R, ...]`) for a given `DefenseSynthConfig`.
pub fn synthesize_defense_tier(cfg: DefenseSynthConfig) -> Vec<i16> {
    let total_frames = (cfg.duration_sec * cfg.sample_rate as f64) as usize;
    let fade_frames = ((cfg.sample_rate as f64 * 0.008) as usize)
        .min(total_frames / 2)
        .max(1);
    let mut escalator = SingleSwitchEscalator::new(cfg.sample_rate);
    let base_delta_r = cfg.effective_delta_r();
    let start_level = cfg.tier.min(6) as f64;

    let mut out = Vec::with_capacity(total_frames * 2);
    for i in 0..total_frames {
        let progress = i as f64 / total_frames.max(1) as f64;
        let (lvl, dr) = if cfg.de_escalate_ramp {
            // Smooth de-escalation ramp from `start_level` -> 0.0:
            // Entropy (dr) collapses faster so cortical anticipation calms down first!
            let rem = (1.0 - progress).max(0.0);
            (start_level * rem, base_delta_r * rem.powf(1.8))
        } else {
            (start_level, base_delta_r)
        };

        let (l, r) = escalator.next_stereo_frame(true, Some((lvl, dr)));
        let env = raised_cosine_env(i, total_frames, fade_frames);
        out.push((l * env * 29500.0) as i16);
        out.push((r * env * 29500.0) as i16);
    }
    out
}

// ============================================================================
//  ZERO-DEPENDENCY RIFF/WAVE 16-BIT PCM ENCODER (WORKS ON ALL TARGETS & WASM)
// ============================================================================

#[inline]
fn raised_cosine_env(idx: usize, total: usize, fade_len: usize) -> f64 {
    if fade_len == 0 || total < fade_len * 2 {
        return 1.0;
    }
    if idx < fade_len {
        0.5 * (1.0 - (PI * idx as f64 / fade_len as f64).cos())
    } else if idx >= total - fade_len {
        let rem = total - 1 - idx;
        0.5 * (1.0 - (PI * rem as f64 / fade_len as f64).cos())
    } else {
        1.0
    }
}

fn normalize_to_i16(samples: &[f64], peak_i16: f64) -> Vec<i16> {
    let max_abs = samples
        .iter()
        .fold(1e-9f64, |acc, &x| acc.max(x.abs()));
    samples
        .iter()
        .map(|&x| ((x / max_abs) * peak_i16).clamp(-32767.0, 32767.0) as i16)
        .collect()
}

/// Encodes 16-bit PCM samples (`mono` when `channels == 1`, interleaved `stereo` when `channels == 2`)
/// into a standard RIFF WAVE byte buffer with zero external crate dependencies.
pub fn encode_wav_pcm16(samples: &[i16], sample_rate: u32, channels: u16) -> Vec<u8> {
    let data_bytes = (samples.len() * 2) as u32;
    let byte_rate = sample_rate * channels as u32 * 2;
    let block_align = channels * 2;
    let riff_size = 36 + data_bytes;

    let mut buf = Vec::with_capacity(44 + data_bytes as usize);
    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&riff_size.to_le_bytes());
    buf.extend_from_slice(b"WAVE");
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&16u32.to_le_bytes()); // PCM chunk size
    buf.extend_from_slice(&1u16.to_le_bytes());  // AudioFormat = 1 (PCM)
    buf.extend_from_slice(&channels.to_le_bytes());
    buf.extend_from_slice(&sample_rate.to_le_bytes());
    buf.extend_from_slice(&byte_rate.to_le_bytes());
    buf.extend_from_slice(&block_align.to_le_bytes());
    buf.extend_from_slice(&16u16.to_le_bytes()); // BitsPerSample = 16
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&data_bytes.to_le_bytes());
    for &s in samples {
        buf.extend_from_slice(&s.to_le_bytes());
    }
    buf
}

// ============================================================================
//  WASM / C FFI EXPORTS (FOR ZERO-DEPENDENCY BROWSER & ANDROID NDK INTEGRATION)
// ============================================================================

static mut LAST_WAV_BUFFER: Option<Vec<u8>> = None;

fn store_wav_result(wav: Vec<u8>) -> *const u8 {
    unsafe {
        let ptr = wav.as_ptr();
        LAST_WAV_BUFFER = Some(wav);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn wasm_last_wav_len() -> usize {
    unsafe { (*std::ptr::addr_of!(LAST_WAV_BUFFER)).as_ref().map(|b| b.len()).unwrap_or(0) }
}

/// Generates a Defense Tier WAV buffer directly in WebAssembly / C FFI memory!
/// `tier`: 0..=4, `delta_r`: 0.0..=1.0 (or -1.0 for default), `duration_sec`: e.g. 4.0,
/// `de_escalate`: 1 for Down-Ramp, 0 for steady/escalating tier.
#[no_mangle]
pub extern "C" fn wasm_render_defense_tier_wav(
    tier: u8,
    delta_r: f64,
    duration_sec: f64,
    de_escalate: u8,
) -> *const u8 {
    let cfg = DefenseSynthConfig {
        tier,
        randomness_delta: delta_r,
        duration_sec: duration_sec.clamp(0.5, 300.0),
        sample_rate: 44100,
        de_escalate_ramp: de_escalate != 0,
    };
    let pcm = synthesize_defense_tier(cfg);
    let wav = encode_wav_pcm16(&pcm, 44100, 2);
    store_wav_result(wav)
}

/// Generates one of the 6 classic Python module presets as a WAV in WASM/FFI memory:
/// `preset_id`:
/// - 0: `dmtf_phone_phreaker.py` (`dtmf_burst.wav`)
/// - 1: `ear_resonance_generator.py` (`canal_resonance_sequence.wav`)
/// - 2: `snappy.py` (`spatial_snap.wav`)
/// - 3: `synth_generator.py` (`implant_dtmf_scale.wav`)
/// - 4: `test_signal.py` (`test_signal.wav`)
/// - 5: `vocoder_bridge.py` (`vocoder_bridge.wav`)
#[no_mangle]
pub extern "C" fn wasm_render_classic_preset_wav(preset_id: u8) -> *const u8 {
    let wav = match preset_id {
        0 => {
            let pcm = generate_tone_sequence("0000000000", 0.1, 0.05, 8000);
            encode_wav_pcm16(&pcm, 8000, 1)
        }
        1 => {
            let pcm = synthesize_ear_pad("123456789*0#", 150, 50, 44100);
            encode_wav_pcm16(&pcm, 44100, 1)
        }
        2 => {
            let pcm = generate_spatial_snap(2.5, 44100);
            encode_wav_pcm16(&pcm, 44100, 2)
        }
        3 => {
            let pcm = generate_implant_tones("137#20", 160, 60, 44100);
            encode_wav_pcm16(&pcm, 44100, 1)
        }
        4 => {
            let pcm = generate_interesting_tone(3.0, 44100);
            encode_wav_pcm16(&pcm, 44100, 1)
        }
        _ => {
            let pcm = generate_vocoder_demo(3.0, 44100);
            encode_wav_pcm16(&pcm, 44100, 1)
        }
    };
    store_wav_result(wav)
}
