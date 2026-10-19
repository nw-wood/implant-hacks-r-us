import numpy as np
from scipy.io.wavfile import write

# DTMF Frequency Map
DTMF_TABLE = {
    '1': (697, 1209), '2': (697, 1336), '3': (697, 1477), 'A': (697, 1633),
    '4': (770, 1209), '5': (770, 1336), '6': (770, 1477), 'B': (770, 1633),
    '7': (852, 1209), '8': (852, 1336), '9': (852, 1477), 'C': (852, 1633),
    '*': (941, 1209), '0': (941, 1336), '#': (941, 1477), 'D': (941, 1633)
}

def generate_tone_sequence(sequence: str, tone_dur: float = 0.1, silence_dur: float = 0.05, sample_rate: int = 8000):
    audio = []
    silence = np.zeros(int(sample_rate * silence_dur))
    
    for char in sequence.upper():
        if char in DTMF_TABLE:
            f_low, f_high = DTMF_TABLE[char]
            t = np.linspace(0, tone_dur, int(sample_rate * tone_dur), endpoint=False)
            # High tone receives slightly more amplitude (+2 dB to +4 dB twist) in standard spec
            tone = 0.5 * np.sin(2 * np.pi * f_low * t) + 0.7 * np.sin(2 * np.pi * f_high * t)
            audio.extend(tone)
            audio.extend(silence)
            
    return np.int16(np.array(audio) * 32767)

# Generate a burst of 0s to overwhelm an IVR buffer
audio_data = generate_tone_sequence("0000000000")
write("dtmf_burst.wav", 8000, audio_data)
