import numpy as np
from scipy.io.wavfile import write

# Tonotopically partitioned matrix based on audio implant channel distributions
IMPLANT_DTMF = {
    '1': (220, 1850), '2': (220, 3000), '3': (220, 4800), 'A': (220, 7200),
    '4': (440, 1850), '5': (440, 3000), '6': (440, 4800), 'B': (440, 7200),
    '7': (730, 1850), '8': (730, 3000), '9': (730, 4800), 'C': (730, 7200),
    '*': (1150, 1850), '0': (1150, 3000), '#': (1150, 4800), 'D': (1150, 7200)
}

def generate_implant_tones(sequence: str, tone_ms: int = 160, gap_ms: int = 60, sr: int = 44100):
    audio = []
    n_tone = int(sr * (tone_ms / 1000.0))
    n_gap = int(sr * (gap_ms / 1000.0))
    silence = np.zeros(n_gap)
    
    # 8ms raised-cosine window to eliminate transient clicking
    fade_len = int(sr * 0.008)
    fade_in = 0.5 * (1 - np.cos(np.linspace(0, np.pi, fade_len)))
    fade_out = 0.5 * (1 + np.cos(np.linspace(0, np.pi, fade_len)))
    env = np.ones(n_tone)
    env[:fade_len] = fade_in
    env[-fade_len:] = fade_out

    t = np.linspace(0, tone_ms / 1000.0, n_tone, endpoint=False)

    for char in sequence.upper():
        if char in IMPLANT_DTMF:
            f_low, f_high = IMPLANT_DTMF[char]
            
            # Fletcher-Munson compensation: low apex frequencies need more drive 
            # than the highly sensitive 3 kHz resonance zone
            amp_low = 0.65
            amp_high = 0.35 if f_high == 3000 else 0.50
            
            tone = (amp_low * np.sin(2 * np.pi * f_low * t) + 
                    amp_high * np.sin(2 * np.pi * f_high * t)) * env
            audio.extend(tone)
            audio.extend(silence)
            
    # Normalize to 16-bit PCM dynamic range
    normalized = np.int16((np.array(audio) / np.max(np.abs(audio))) * 30000)
    return normalized

# Generate a sequence dialing through the four corners and center of the basilar map
dial_string = "137#20"
audio_buffer = generate_implant_tones(dial_string)
write("implant_dtmf_scale.wav", 44100, audio_buffer)
