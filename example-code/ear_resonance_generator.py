import numpy as np
from scipy.io.wavfile import write

EAR_RESONANCE_DTMF = {
    '1': (2700, 4000), '2': (2700, 4500), '3': (2700, 5000), 'A': (2700, 5500),
    '4': (3100, 4000), '5': (3100, 4500), '6': (3100, 5000), 'B': (3100, 5500),
    '7': (3500, 4000), '8': (3500, 4500), '9': (3500, 5000), 'C': (3500, 5500),
    '*': (3900, 4000), '0': (3900, 4500), '#': (3900, 5000), 'D': (3900, 5500)
}

def synthesize_ear_pad(sequence: str, tone_ms: int = 150, gap_ms: int = 50, sr: int = 44100):
    audio = []
    n_tone = int(sr * (tone_ms / 1000.0))
    n_gap = int(sr * (gap_ms / 1000.0))
    silence = np.zeros(n_gap)
    
    # 5ms cosine fade on/off to avoid speaker clicks
    fade_len = int(sr * 0.005)
    fade_in = 0.5 * (1 - np.cos(np.linspace(0, np.pi, fade_len)))
    fade_out = 0.5 * (1 + np.cos(np.linspace(0, np.pi, fade_len)))
    env = np.ones(n_tone)
    env[:fade_len] = fade_in
    env[-fade_len:] = fade_out

    t = np.linspace(0, tone_ms / 1000.0, n_tone, endpoint=False)

    for char in sequence.upper():
        if char in EAR_RESONANCE_DTMF:
            f1, f2 = EAR_RESONANCE_DTMF[char]
            tone = 0.5 * np.sin(2 * np.pi * f1 * t) + 0.5 * np.sin(2 * np.pi * f2 * t)
            tone *= env
            audio.extend(tone)
            audio.extend(silence)
            
    # Scaled conservatively so it doesn't pierce eardrums unexpectedly
    return np.int16(np.array(audio) * 12000)

buf = synthesize_ear_pad("123456789*0#")
write("canal_resonance_sequence.wav", 44100, buf)
