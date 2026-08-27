use crate::models::audio::{Phoneme, PhonemeMap};

pub struct AudioSyncService;

impl AudioSyncService {
    pub fn map_phonemes_to_frames(
        &self,
        phonemes: &[Phoneme],
        frame_rate: u32,
        duration_ms: u64,
    ) -> PhonemeMap {
        let frame_count = (duration_ms as f32 / 1000.0 * frame_rate as f32).ceil() as usize;
        let mut map = PhonemeMap::new(frame_count);

        for p in phonemes {
            let frame_index = ((p.timestamp_ms as f32 / 1000.0) * frame_rate as f32).floor() as usize;
            if frame_index < frame_count {
                map.set_phoneme(frame_index, p.code.clone());
            }
        }

        map
    }
}
