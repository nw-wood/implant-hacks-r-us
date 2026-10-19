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
//  IMPLANT-HACKS-R-US: MULTI-PLATFORM ENTRY POINT
//  "Hooked on Phon-et-ics worked for me! Dial 1-800-ABCDEFG to claim your
//   complimentary consonant blend and silence unwanted neural chatter!"
// ============================================================================

use implant_hacks_r_us::{
    encode_wav_pcm16, generate_implant_tones, generate_interesting_tone, generate_keypad_sequence,
    generate_spatial_snap, generate_tone_sequence, generate_vocoder_demo, synthesize_defense_tier,
    synthesize_ear_pad, DefenseSynthConfig, KeypadType, SingleSwitchEscalator,
};
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Embedded single-file Web Studio UI so the compiled binary requires ZERO external files!
const EMBEDDED_WEB_UI: &str = include_str!("../web/index.html");

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("serve");

    match cmd {
        "--help" | "-h" | "help" => print_help(),
        "--export-all" | "export" | "generate" => {
            let out_dir = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            export_all_wavs(out_dir);
        }
        "--switch-mode" | "switch" => {
            let gpio_path = args
                .get(2)
                .map(|s| s.as_str())
                .unwrap_or("/sys/class/gpio/gpio17/value");
            run_hardware_switch_daemon(gpio_path);
        }
        "serve" | "--serve" | _ => {
            let port: u16 = args
                .get(2)
                .and_then(|s| s.parse().ok())
                .unwrap_or(8080);
            run_embedded_ui_server(port);
        }
    }
}

fn print_help() {
    println!("======================================================================");
    println!(" IMPLANT-HACKS-R-US v0.1.0 // Hooked on Phon-et-ics DSP Toolkit");
    println!(" Copyright (C) 2026 NATHANYEL WOOD <nathanyelw@gmail.com>");
    println!(" License: GNU GPLv3 or later (GPL-3.0-or-later)");
    println!("======================================================================");
    println!("Usage:");
    println!("  implant-hacks-r-us [serve] [port]       Launch self-contained Web/GUI Studio (default)");
    println!("  implant-hacks-r-us --export-all [dir]   Export all 6 classic + 5 Tier + Hot Cross Buns WAVs");
    println!("  implant-hacks-r-us --switch-mode [pin]  Run single-switch metal escalator (5-min de-escalation)");
    println!();
    println!("Remember: Sound out your digraphs! 1-800-ABCDEFG!");
}

fn save_wav(dir: &Path, filename: &str, pcm: &[i16], sr: u32, channels: u16) {
    let path = dir.join(filename);
    let wav_bytes = encode_wav_pcm16(pcm, sr, channels);
    fs::write(&path, wav_bytes).expect("Failed to write WAV file");
    println!("  [★ GOLD STAR] Wrote {}", path.display());
}

fn export_all_wavs(out_dir: &str) {
    let dir = Path::new(out_dir);
    fs::create_dir_all(dir).expect("Failed to create output directory");
    println!(":: Hooked on Phon-et-ics Batch WAV Synthesizer starting...");

    // 1-6: Exact outputs of the 6 Python scripts in example-code/
    save_wav(
        dir,
        "dtmf_burst.wav",
        &generate_tone_sequence("0000000000", 0.1, 0.05, 8000),
        8000,
        1,
    );
    save_wav(
        dir,
        "canal_resonance_sequence.wav",
        &synthesize_ear_pad("123456789*0#", 150, 50, 44100),
        44100,
        1,
    );
    save_wav(
        dir,
        "spatial_snap.wav",
        &generate_spatial_snap(2.5, 44100),
        44100,
        2,
    );
    save_wav(
        dir,
        "implant_dtmf_scale.wav",
        &generate_implant_tones("137#20", 160, 60, 44100),
        44100,
        1,
    );
    save_wav(
        dir,
        "test_signal.wav",
        &generate_interesting_tone(3.0, 44100),
        44100,
        1,
    );
    save_wav(
        dir,
        "vocoder_bridge.wav",
        &generate_vocoder_demo(3.0, 44100),
        44100,
        1,
    );

    // 7-8: Silly Sub-100 Hz "Hot Cross Buns" (Raw Infrasonic & Tartini Phantom Beat)
    save_wav(
        dir,
        "hot_cross_buns_sub100_ghost.wav",
        &generate_keypad_sequence(
            KeypadType::Sub100Ghost,
            "321 321 11112222321",
            180,
            60,
            44100,
            0.0,
        ),
        44100,
        1,
    );
    save_wav(
        dir,
        "hot_cross_buns_tartini_phantom.wav",
        &generate_keypad_sequence(
            KeypadType::Sub100TartiniPhantom,
            "321 321 11112222321",
            180,
            60,
            44100,
            0.0,
        ),
        44100,
        1,
    );

    // 9-13: Combined Defense Tiers 0..=4 with escalating Randomness Delta & Haas Dilation
    for tier in 0..=4 {
        let pcm = synthesize_defense_tier(DefenseSynthConfig {
            tier,
            randomness_delta: -1.0, // auto tier delta_r
            duration_sec: 5.0,
            sample_rate: 44100,
            de_escalate_ramp: false,
        });
        save_wav(
            dir,
            &format!("defense_tier_{}.wav", tier),
            &pcm,
            44100,
            2,
        );
    }

    // 14: Smooth De-Escalation Down-Ramp WAV
    let deesc_pcm = synthesize_defense_tier(DefenseSynthConfig {
        tier: 4,
        randomness_delta: 0.95,
        duration_sec: 10.0,
        sample_rate: 44100,
        de_escalate_ramp: true,
    });
    save_wav(dir, "de_escalation_down_ramp.wav", &deesc_pcm, 44100, 2);

    println!(":: All 14 Phonics WAV files exported to '{}'!", dir.display());
}

/// Embedded / Raspberry Pi / PinePhone 64 Single-Switch Escalator Daemon:
/// Reads a hardware GPIO switch (`1` = ON / Escalate, `0` = OFF / 5-Minute 300s De-Escalation)
/// and streams real-time stereo 16-bit 44.1 kHz audio to `pw-play` or `aplay`.
fn run_hardware_switch_daemon(gpio_value_path: &str) {
    println!(":: [METAL SWITCH MODE] Monitoring switch at: {}", gpio_value_path);
    println!(":: Switch ON  -> Sliding escalation (0.0 -> 4.0) + Randomness Delta + Haas Dilation");
    println!(":: Switch OFF -> 5-Minute (300.0s) smooth de-escalation ramp to silence");

    let sr = 44100u32;
    let mut escalator = SingleSwitchEscalator::new(sr);
    escalator.de_escalation_time_sec = 300.0; // Strictly 5 minutes as required!

    // Try spawning aplay or pw-play for zero-config Linux/RPi audio output
    let mut player = Command::new("aplay")
        .args(["-f", "S16_LE", "-c", "2", "-r", "44100", "-q", "-"])
        .stdin(Stdio::piped())
        .spawn()
        .or_else(|_| {
            Command::new("pw-play")
                .args(["--rate=44100", "--channels=2", "--format=s16", "-"])
                .stdin(Stdio::piped())
                .spawn()
        })
        .ok();

    let chunk_frames = 2205usize; // 50ms audio blocks
    let mut byte_buf = Vec::with_capacity(chunk_frames * 4);
    let mut last_log = Instant::now();

    loop {
        let switch_on = fs::read_to_string(gpio_value_path)
            .map(|s| s.trim() == "1" || s.trim().eq_ignore_ascii_case("on"))
            .unwrap_or(true);

        byte_buf.clear();
        for _ in 0..chunk_frames {
            let (l, r) = escalator.next_stereo_frame(switch_on, None);
            let li = (l * 28000.0) as i16;
            let ri = (r * 28000.0) as i16;
            byte_buf.extend_from_slice(&li.to_le_bytes());
            byte_buf.extend_from_slice(&ri.to_le_bytes());
        }

        if let Some(ref mut child) = player {
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(&byte_buf);
            }
        } else {
            std::thread::sleep(Duration::from_millis(50));
        }

        if last_log.elapsed() >= Duration::from_secs(2) {
            let rem_sec = (escalator.current_level / 4.0) * 300.0;
            println!(
                "  [Switch: {}] Level: {:.2}/4.00 | 5-Min De-Escalation Remaining: {:.1}s",
                if switch_on { "ON " } else { "OFF" },
                escalator.current_level,
                rem_sec
            );
            last_log = Instant::now();
        }
    }
}

fn run_embedded_ui_server(preferred_port: u16) {
    let listener = TcpListener::bind(("127.0.0.1", preferred_port))
        .or_else(|_| TcpListener::bind(("127.0.0.1", 0)))
        .expect("Failed to bind local UI server port");
    let local_addr = listener.local_addr().unwrap();
    let url = format!("http://{}", local_addr);

    println!("======================================================================");
    println!(" IMPLANT-HACKS-R-US // Hooked on Phon-et-ics Studio is LIVE!");
    println!(" Open in browser: {}", url);
    println!(" (Press Ctrl+C to stop)");
    println!("======================================================================");

    // Proactively open browser on desktop/PinePhone if xdg-open is present
    let _ = Command::new("xdg-open").arg(&url).spawn();

    for stream in listener.incoming().flatten() {
        let mut stream = stream;
        let mut req_buf = [0u8; 2048];
        let _ = stream.read(&mut req_buf);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n{}",
            EMBEDDED_WEB_UI.len(),
            EMBEDDED_WEB_UI
        );
        let _ = stream.write_all(response.as_bytes());
        let _ = stream.flush();
    }
}
