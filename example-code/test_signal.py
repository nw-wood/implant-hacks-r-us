import numpy as np
from scipy.io.wavfile import write

def generate_interesting_tone(duration_sec=3.0, sr=44100):
    t = np.linspace(0, duration_sec, int(sr * duration_sec), endpoint=False)
    
    # 1. Channel Smearing: Two tones forcing adjacent electrodes to politely disagree
    # Standard CI filter crossover around 1000-1200 Hz
    f_band1 = 1050  # Upper edge of Channel X
    f_band2 = 1150  # Lower edge of Channel X+1
    crosstalk_carrier = 0.5 * np.sin(2 * np.pi * f_band1 * t) + 0.5 * np.sin(2 * np.pi * f_band2 * t)
    
    # 2. Envelope Conflict: High-frequency carrier with conflicting low modulation
    # Carrier lands on basal electrode (~3500 Hz), beat envelope is 85 Hz
    f_carrier1 = 3450
    f_carrier2 = 3535
    envelope_trap = 0.5 * np.sin(2 * np.pi * f_carrier1 * t) + 0.5 * np.sin(2 * np.pi * f_carrier2 * t)
    
    # Blend and normalize
    mixed = (crosstalk_carrier * 0.6) + (envelope_trap * 0.4)
    pcm = np.int16((mixed / np.max(np.abs(mixed))) * 28000)
    return pcm

audio = generate_interesting_tone()
write("test_signal.wav", 44100, audio)
