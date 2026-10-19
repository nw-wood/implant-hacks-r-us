// Copyright (C) 2026 NATHANYEL WOOD <nathanyelw@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// ============================================================================
//  ESP32 SINGLE-SWITCH PHONICS ESCALATOR + 5-MINUTE DE-ESCALATION FIRMWARE
//  "Hooked on Phon--et--ics worked for me! Dial 1-800-ABCDEFG!"
//
//  PINOUT (NO KEYPADS - SINGLE HARDWARE SWITCH):
//    - GPIO 4 (INPUT_PULLUP -> GND): Single Escalator Switch
//        * Switch ON (LOW): Escalates continuously 0.0 -> 4.0 (Randomness Delta
//          + Randomized Implant/Canal/DTMF Registers + Haas Spatial Dilation)
//        * Switch OFF (HIGH): De-escalates smoothly over 5 MINUTES (300.0s)!
//    - GPIO 25: Built-in DAC Channel 1 (Left Audio Out)
//    - GPIO 26: Built-in DAC Channel 2 (Right Audio Out)
// ============================================================================

#include <Arduino.h>
#include <driver/dac.h>
#include <math.h>

#define SWITCH_GPIO      4
#define SAMPLE_RATE      22050.0f
#define ESCALATE_SEC     30.0f
#define DEESCALATE_SEC   300.0f   // 5 MINUTES (300.0s) smooth de-escalation!

static const float IMPLANT_LOW[4]  = { 220.0f, 440.0f, 730.0f, 1150.0f };
static const float IMPLANT_HIGH[4] = { 1850.0f, 3000.0f, 4800.0f, 7200.0f };
static const float CANAL_LOW[4]    = { 2700.0f, 3100.0f, 3500.0f, 3900.0f };
static const float CANAL_HIGH[4]   = { 4000.0f, 4500.0f, 5000.0f, 5500.0f };

static float current_level = 0.0f;
static uint32_t sample_clock = 0;
static uint32_t reg_timer = 1;
static bool in_gap = false;
static float imp_f1 = 220.0f, imp_f2 = 3000.0f;
static float can_f1 = 3100.0f, can_f2 = 4500.0f;
static uint32_t haas_timer = 11025;
static int8_t haas_dir = 0;
static uint8_t haas_lag = 55;
static float delay_buf[128];
static uint8_t delay_idx = 0;

static float grey_lp = 0.0f, grey_mid = 0.0f;

void setup() {
  pinMode(SWITCH_GPIO, INPUT_PULLUP);
  dac_output_enable(DAC_CHANNEL_1); // GPIO 25 (Left)
  dac_output_enable(DAC_CHANNEL_2); // GPIO 26 (Right)
}

void loop() {
  const float up_step   = 5.0f / (ESCALATE_SEC * SAMPLE_RATE);
  const float down_step = 5.0f / (DEESCALATE_SEC * SAMPLE_RATE);
  const uint32_t us_per_sample = (uint32_t)(1000000.0f / SAMPLE_RATE);
  uint32_t next_tick = micros();

  while (true) {
    bool sw_on = (digitalRead(SWITCH_GPIO) == LOW);
    if (sw_on) {
      current_level = fminf(5.0f, current_level + up_step);
    } else {
      current_level = fmaxf(0.0f, current_level - down_step);
    }

    if (current_level <= 0.0005f) {
      dac_output_voltage(DAC_CHANNEL_1, 128);
      dac_output_voltage(DAC_CHANNEL_2, 128);
      delay(25);
      next_tick = micros();
      continue;
    }

    float lvl = current_level;
    float dr  = fminf(1.0f, (lvl / 4.0f) * 0.98f);
    float t   = (float)(sample_clock++) / SAMPLE_RATE;

    if (reg_timer == 0) {
      in_gap = !in_gap;
      if (!in_gap) {
        uint32_t r = esp_random();
        imp_f1 = IMPLANT_LOW[r & 3];       imp_f2 = IMPLANT_HIGH[(r >> 2) & 3];
        can_f1 = CANAL_LOW[(r >> 4) & 3];  can_f2 = CANAL_HIGH[(r >> 6) & 3];
        reg_timer = (uint32_t)((0.05f + dr * ((r & 0xFF) / 255.0f) * 0.20f) * SAMPLE_RATE);
      } else {
        reg_timer = (uint32_t)((0.025f + dr * ((esp_random() & 0xFF) / 255.0f) * 0.35f) * SAMPLE_RATE);
      }
    } else {
      reg_timer--;
    }

    float haas_w = fminf(1.0f, fmaxf(0.0f, lvl - 3.0f));
    if (haas_w > 0.01f) {
      if (haas_timer == 0) {
        uint32_t r = esp_random();
        haas_timer = (uint32_t)((0.15f + ((r & 0xFF) / 255.0f) * 1.2f * dr) * SAMPLE_RATE);
        haas_dir = ((r >> 8) & 1) ? 1 : -1;
        haas_lag = 35 + ((r >> 10) & 63);
      } else {
        haas_timer--;
      }
    }

    float gate = in_gap ? 0.0f : 1.0f;
    float env4 = 0.5f * (1.0f + sinf(6.2831853f * 4.0f * t));
    float harm = (0.5f * sinf(6.2831853f * 300.0f * t) + 0.3f * sinf(6.2831853f * 600.0f * t)) * env4;
    float smear = 0.5f * sinf(6.2831853f * 1050.0f * t) + 0.5f * sinf(6.2831853f * 1150.0f * t);
    float imp = (0.6f * sinf(6.2831853f * imp_f1 * t) + 0.4f * sinf(6.2831853f * imp_f2 * t)) * gate;
    float can = (0.5f * sinf(6.2831853f * can_f1 * t) + 0.5f * sinf(6.2831853f * can_f2 * t)) * gate;

    // Tier 5 Secret Black Workbook: Continuous 220 Hz + 7200 Hz rail lock + Grey Noise static
    float w_t5 = fminf(1.0f, fmaxf(0.0f, lvl - 4.0f));
    float black_book = 0.0f;
    if (w_t5 > 0.001f) {
      float rails = 0.65f * sinf(6.2831853f * 220.0f * t) + 0.55f * sinf(6.2831853f * 7200.0f * t);
      float white = ((int32_t)(esp_random() & 0xFFFF) - 32768) / 32768.0f;
      grey_lp  += 0.06f * (white - grey_lp);
      grey_mid += 0.38f * (white - grey_mid);
      float grey = grey_lp * 0.65f + white * 0.22f + (white - grey_mid) * 0.55f;
      black_book = w_t5 * (rails * 0.52f + grey * 0.38f);
    }

    float bg_scale = 1.0f - (w_t5 * 0.28f);
    float mono = (0.3f * harm + fminf(1.0f, lvl) * 0.3f * smear
               + fminf(1.0f, fmaxf(0.0f, lvl - 1.0f)) * 0.2f * imp
               + fminf(1.0f, fmaxf(0.0f, lvl - 2.0f)) * 0.2f * can) * bg_scale
               + black_book;

    delay_buf[delay_idx] = mono;
    uint8_t d_idx = (delay_idx + 128 - haas_lag) & 127;
    delay_idx = (delay_idx + 1) & 127;

    float outL = mono, outR = mono;
    if (haas_w > 0.01f) {
      if (haas_dir > 0) outL = delay_buf[d_idx] * 0.25f;
      else              outR = delay_buf[d_idx] * 0.25f;
    }

    uint8_t dacL = (uint8_t)fmaxf(0.0f, fminf(255.0f, 128.0f + outL * 110.0f));
    uint8_t dacR = (uint8_t)fmaxf(0.0f, fminf(255.0f, 128.0f + outR * 110.0f));
    dac_output_voltage(DAC_CHANNEL_1, dacL);
    dac_output_voltage(DAC_CHANNEL_2, dacR);

    next_tick += us_per_sample;
    int32_t wait_us = (int32_t)(next_tick - micros());
    if (wait_us > 0) delayMicroseconds(wait_us);
  }
}
