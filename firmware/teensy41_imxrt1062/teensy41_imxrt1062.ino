// Copyright (C) 2026 NATHANYEL WOOD <nathanyelw@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// ============================================================================
//  TEENSY 4.1 (NXP i.MXRT1062 @ 600 MHz Cortex-M7) BARE-METAL ESCALATOR
//  "Hooked on Phon--et--ics worked for me! 1-800-ABCDEFG!"
//
//  HARDWARE CONTROLS (NO KEYPADS - SINGLE SLIDING/TOGGLE SWITCH):
//    - Pin 2 (INPUT_PULLUP -> GND): Main Escalator Switch
//        * Switch CLOSED (LOW / ON): Slides up the Defense Escalation Ladder
//          (0.0 -> 4.0), ramping up Randomness Delta (dR), randomizing
//          IMPLANT_DTMF + EAR_RESONANCE_DTMF + DTMF_TABLE electrode registers,
//          and dilating Haas Precedence L/R spatial snaps (1.6ms - 4.4ms).
//        * Switch OPEN (HIGH / OFF): Smoothly de-escalates over exactly
//          5 MINUTES (300.0 seconds) back down to Silent 'E' grounding & mute!
//    - Pin 13: Onboard LED (blinks at syllabic/register rate; solid when muted)
//    - Audio Out: Teensy Audio Shield I2S OR MQS Stereo (Pin 10 Left, Pin 12 Right)
// ============================================================================

#include <Arduino.h>
#include <Audio.h>
#include <math.h>

#define SWITCH_PIN       2
#define LED_PIN          13
#define SAMPLE_RATE      44100.0f
#define ESCALATE_SEC     30.0f    // 30 seconds to slide from 0.0 -> 4.0 when ON
#define DEESCALATE_SEC   300.0f   // 5 MINUTES (300.0s) de-escalation when OFF!

// Phonics Frequency Tables (from synth_generator.py, ear_resonance_generator.py, dmtf_phone_phreaker.py)
static const float IMPLANT_LOW[4]  = { 220.0f, 440.0f, 730.0f, 1150.0f };
static const float IMPLANT_HIGH[4] = { 1850.0f, 3000.0f, 4800.0f, 7200.0f };
static const float CANAL_LOW[4]    = { 2700.0f, 3100.0f, 3500.0f, 3900.0f };
static const float CANAL_HIGH[4]   = { 4000.0f, 4500.0f, 5000.0f, 5500.0f };
static const float DTMF_LOW[4]     = { 697.0f, 770.0f, 852.0f, 941.0f };
static const float DTMF_HIGH[4]    = { 1209.0f, 1336.0f, 1477.0f, 1633.0f };

class AudioSynthPhonicsEscalator : public AudioStream {
public:
  AudioSynthPhonicsEscalator() : AudioStream(0, NULL) {
    current_level = 0.0f;
    sample_clock = 0;
    rng_state = 0x1800ABCDEF2026ULL;
    reg_timer = 1;
    in_gap = false;
    haas_timer = 22050;
    haas_dir = 0;
    haas_lag = 110;
    haas_atten = 0.25f;
    delay_idx = 0;
    memset(delay_l, 0, sizeof(delay_l));
    memset(delay_r, 0, sizeof(delay_r));
  }

  volatile bool switch_engaged = false;
  volatile float current_level;

  uint32_t xorshift32() {
    rng_state ^= rng_state >> 12;
    rng_state ^= rng_state << 25;
    rng_state ^= rng_state >> 27;
    return (uint32_t)((rng_state * 0x2545F4914F6CDD1DULL) >> 32);
  }

  float rand_f32() {
    return (float)(xorshift32() & 0x00FFFFFF) / 16777216.0f;
  }

  virtual void update(void) override {
    audio_block_t *blockL = allocate();
    audio_block_t *blockR = allocate();
    if (!blockL || !blockR) {
      if (blockL) release(blockL);
      if (blockR) release(blockR);
      return;
    }

    const float up_step   = 6.0f / (ESCALATE_SEC * SAMPLE_RATE);
    const float down_step = 6.0f / (DEESCALATE_SEC * SAMPLE_RATE);

    for (int i = 0; i < AUDIO_BLOCK_SAMPLES; i++) {
      if (switch_engaged) {
        current_level = fminf(6.0f, current_level + up_step);
      } else {
        current_level = fmaxf(0.0f, current_level - down_step);
      }

      if (current_level <= 0.0005f) {
        blockL->data[i] = 0;
        blockR->data[i] = 0;
        continue;
      }

      float lvl = current_level;
      float dr  = fminf(1.0f, powf(fminf(1.0f, lvl / 4.0f), 1.35f) * 0.98f);
      float t   = (float)(sample_clock++) / SAMPLE_RATE;

      // Register Hopping with Randomness Delta (dr)
      if (reg_timer == 0) {
        in_gap = !in_gap;
        if (!in_gap) {
          uint32_t r1 = xorshift32(), r2 = xorshift32(), r3 = xorshift32();
          imp_f1  = IMPLANT_LOW[r1 & 3];  imp_f2  = IMPLANT_HIGH[(r1 >> 2) & 3];
          can_f1  = CANAL_LOW[r2 & 3];    can_f2  = CANAL_HIGH[(r2 >> 2) & 3];
          dtmf_f1 = DTMF_LOW[r3 & 3];     dtmf_f2 = DTMF_HIGH[(r3 >> 2) & 3];
          float ms = 140.0f + dr * (rand_f32() * 2.0f - 1.0f) * 110.0f;
          reg_timer = (uint32_t)(fmaxf(35.0f, ms) * (SAMPLE_RATE / 1000.0f));
        } else {
          float gap_ms = (dr > 0.5f) ? (25.0f + rand_f32() * 450.0f * dr) : 55.0f;
          reg_timer = (uint32_t)(gap_ms * (SAMPLE_RATE / 1000.0f));
        }
      } else {
        reg_timer--;
      }

      // Haas Spatial Dilation (Tier 3 -> 6)
      float haas_w = fminf(1.0f, fmaxf(0.0f, lvl - 3.0f));
      if (haas_w > 0.01f) {
        if (haas_timer == 0) {
          float wait_s = 0.12f + rand_f32() * (0.4f + 1.25f * dr);
          haas_timer = (uint32_t)(wait_s * SAMPLE_RATE);
          float roll = rand_f32();
          haas_dir = (roll < 0.44f) ? 1 : ((roll < 0.88f) ? -1 : 0);
          float lag_s = 0.0016f + rand_f32() * 0.0028f * fmaxf(0.3f, dr);
          haas_lag = (uint16_t)fminf(240.0f, fmaxf(1.0f, lag_s * SAMPLE_RATE));
          haas_atten = 0.16f + (1.0f - dr) * 0.15f;
        } else {
          haas_timer--;
        }
      } else {
        haas_dir = 0;
      }

      float env4 = 0.5f * (1.0f + sinf(6.2831853f * 4.0f * t));
      float harm = (0.5f * sinf(6.2831853f * 300.0f * t)
                  + 0.3f * sinf(6.2831853f * 600.0f * t)
                  + 0.2f * sinf(6.2831853f * 900.0f * t)) * env4;
      float smear = 0.5f * sinf(6.2831853f * 1050.0f * t) + 0.5f * sinf(6.2831853f * 1150.0f * t);
      float w_smear = fminf(1.0f, lvl) * 0.5f;
      float w_t2 = fminf(1.0f, fmaxf(0.0f, lvl - 1.0f));
      float w_t3 = fminf(1.0f, fmaxf(0.0f, lvl - 2.0f));
      float w_t5 = fminf(1.0f, fmaxf(0.0f, lvl - 4.0f));
      float w_t6 = fminf(1.0f, fmaxf(0.0f, lvl - 5.0f));
      float gate = in_gap ? 0.0f : 1.0f;

      float imp = (0.65f * sinf(6.2831853f * imp_f1 * t) + 0.40f * sinf(6.2831853f * imp_f2 * t)) * gate;
      float can = (0.50f * sinf(6.2831853f * can_f1 * t) + 0.50f * sinf(6.2831853f * can_f2 * t)) * gate;
      float dtm = (0.50f * sinf(6.2831853f * dtmf_f1 * t) + 0.70f * sinf(6.2831853f * dtmf_f2 * t)) * gate;
      float anchor = 0.5f * sinf(6.2831853f * 730.0f * t) + 0.5f * sinf(6.2831853f * 3000.0f * t);

      // Tier 5 Secret Black Workbook: Continuous un-gated 220 Hz + 7200 Hz rail lock + Grey Noise static
      float black_book = 0.0f;
      if (w_t5 > 0.001f) {
        float rails = 0.65f * sinf(6.2831853f * 220.0f * t) + 0.55f * sinf(6.2831853f * 7200.0f * t);
        float white = rand_f32() * 2.0f - 1.0f;
        grey_lp  += 0.045f * (white - grey_lp);
        grey_mid += 0.32f  * (white - grey_mid);
        float grey = grey_lp * 0.65f + white * 0.22f + (white - grey_mid) * 0.55f;
        black_book = w_t5 * (rails * 0.52f + grey * 0.38f);
      }

      // Tier 6: "I don't care anymore eat puppies" — Continuous un-gated 1000 Hz Constant Wave!
      float eat_puppies_1000hz = (w_t6 > 0.001f) ? (w_t6 * 0.65f * sinf(6.2831853f * 1000.0f * t)) : 0.0f;

      float bg_scale = 1.0f - (w_t5 * 0.28f);
      float mono = (0.35f * harm + w_smear * smear * 0.4f
                 + w_t2 * imp * 0.35f + w_t3 * (can * 0.3f + dtm * 0.3f)
                 + haas_w * anchor * 0.4f) * bg_scale + black_book + eat_puppies_1000hz;

      delay_l[delay_idx] = mono;
      delay_r[delay_idx] = mono;
      uint16_t d_idx = (delay_idx + 256 - haas_lag) & 255;
      delay_idx = (delay_idx + 1) & 255;

      float outL = mono, outR = mono;
      if (haas_w > 0.01f && haas_dir != 0) {
        if (haas_dir > 0) outL = mono * (1.0f - haas_w) + (delay_l[d_idx] * haas_atten) * haas_w;
        else              outR = mono * (1.0f - haas_w) + (delay_r[d_idx] * haas_atten) * haas_w;
      }

      blockL->data[i] = (int16_t)(fmaxf(-1.0f, fminf(1.0f, outL * 0.5f)) * 28000.0f);
      blockR->data[i] = (int16_t)(fmaxf(-1.0f, fminf(1.0f, outR * 0.5f)) * 28000.0f);
    }

    transmit(blockL, 0);
    transmit(blockR, 1);
    release(blockL);
    release(blockR);
  }

private:
  uint64_t rng_state;
  uint64_t sample_clock;
  uint32_t reg_timer;
  bool in_gap;
  float imp_f1 = 220.0f, imp_f2 = 3000.0f;
  float can_f1 = 3100.0f, can_f2 = 4500.0f;
  float dtmf_f1 = 941.0f, dtmf_f2 = 1336.0f;
  float grey_lp = 0.0f, grey_mid = 0.0f;
  uint32_t haas_timer;
  int8_t haas_dir;
  uint16_t haas_lag;
  float haas_atten;
  float delay_l[256];
  float delay_r[256];
  uint16_t delay_idx;
};

AudioSynthPhonicsEscalator phonicsSynth;
AudioOutputMQS             mqsOut;       // Direct stereo MQS on Teensy 4.1 Pins 10 & 12
AudioConnection            patchCordL(phonicsSynth, 0, mqsOut, 0);
AudioConnection            patchCordR(phonicsSynth, 1, mqsOut, 1);

void setup() {
  pinMode(SWITCH_PIN, INPUT_PULLUP);
  pinMode(LED_PIN, OUTPUT);
  AudioMemory(16);
}

void loop() {
  // Active-LOW toggle switch on Pin 2
  bool sw_on = (digitalRead(SWITCH_PIN) == LOW);
  phonicsSynth.switch_engaged = sw_on;
  digitalWrite(LED_PIN, sw_on ? HIGH : (phonicsSynth.current_level > 0.01f ? (millis() / 250) % 2 : LOW));
  delay(20);
}
