use uuid::Uuid;
use super::audio_waveform::AudioWaveform;

#[derive(Debug, Clone)]
pub struct AudioTrack {
    pub id: Uuid,
    pub waveform: AudioWaveform,
}

impl AudioTrack {
    pub fn new(waveform: AudioWaveform) -> Self {
        Self {
            id: Uuid::new_v4(),
            waveform,
        }
    }
}
