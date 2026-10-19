import numpy as np
from scipy.io.wavfile import write

def generate_spatial_snap(dur_sec=2.5, sr=44100):
    total_samples = int(dur_sec * sr)
    t = np.linspace(0, dur_sec, total_samples, endpoint=False)
    
    # Base tone: Formant anchor (730 Hz + 3000 Hz) for maximum loudness
    carrier = 0.5 * np.sin(2 * np.pi * 730 * t) + 0.5 * np.sin(2 * np.pi * 3000 * t)
    
    # Initialize stereo channels
    left = np.copy(carrier)
    right = np.copy(carrier)
    
    # At t = 1.2 seconds, trigger a sudden 2.5ms Precedence (Haas) lag on the Left channel
    # and a 12 dB drop, snapping the acoustic focus hard to the right
    snap_idx = int(1.2 * sr)
    lag_samples = int(0.0025 * sr)  # 2.5 ms delay (~110 samples)
    
    # Apply Haas delay and level attenuation to Left channel after snap point
    left_delayed = np.zeros_like(left)
    left_delayed[:snap_idx] = left[:snap_idx]
    
    # Lag the remainder of the left audio stream and attenuate
    left_tail = left[snap_idx:-lag_samples] * 0.25  # -12 dB attenuation
    left_delayed[snap_idx + lag_samples:] = left_tail
    
    # Keep right channel full strength and un-delayed
    right_processed = np.copy(right)
    
    # Stack into stereo array (Left, Right)
    stereo = np.vstack((left_delayed, right_processed)).T
    
    # 5ms fade-in/out to prevent start clicks
    fade_len = int(sr * 0.005)
    fade_env = np.ones(total_samples)
    fade_env[:fade_len] = np.linspace(0, 1, fade_len)
    fade_env[-fade_len:] = np.linspace(1, 0, fade_len)
    
    stereo[:, 0] *= fade_env
    stereo[:, 1] *= fade_env
    
    # Normalize to 16-bit PCM
    stereo_pcm = np.int16((stereo / np.max(np.abs(stereo))) * 30000)
    return stereo_pcm

stereo_audio = generate_spatial_snap()
write("spatial_snap.wav", 44100, stereo_audio)
