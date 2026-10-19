import numpy as np
from scipy.io.wavfile import write

def generate__vocoder_demo(dur=3.0, sr=44100):
    t = np.linspace(0, dur, int(sr * dur), endpoint=False)
    
    # 1. Create a dynamic modulation envelope (a slow 4 Hz tremolo sweep)
    mod_env = 0.5 * (1 + np.sin(2 * np.pi * 4 * t))
    
    # 2. Carrier A: Pure biological fine-structure (harmonic richness)
    # Natural sound with fundamental and integer harmonics
    carrier = (0.5 * np.sin(2 * np.pi * 300 * t) + 
                   0.3 * np.sin(2 * np.pi * 600 * t) + 
                   0.2 * np.sin(2 * np.pi * 900 * t))
    signal = carrier * mod_env
    
    # 3. Carrier B: Implant stimulation proxy (envelope-modulated pulse train)
    # The implant ignores phase and converts the envelope into discrete current bursts
    pulse_rate = 900  # 900 pulses per second (typical CIS stimulation rate)
    pulse_train = np.where(np.sin(2 * np.pi * pulse_rate * t) > 0.9, 1.0, 0.0)
    proxy_signal = pulse_train * mod_env
    
    # Concatenate: 1.5s biological harmonic form -> 1.5s raw electrode pulse firing
    half = len(t) // 2
    output = np.concatenate([signal[:half], proxy_signal[half:]])
    
    # Normalize to 16-bit PCM
    output_pcm = np.int16((output / np.max(np.abs(output))) * 28000)
    return output_pcm

buf = generate_vocoder_demo()
write("vocoder_bridge.wav", 44100, buf)
