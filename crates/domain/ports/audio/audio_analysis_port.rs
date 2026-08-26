use crate::domain::models::audio::Phoneme;

/// Port métier : analyse audio (phonèmes).
pub trait AudioAnalysisPort {
    fn analyze_phonemes(&self, audio_path: &str) -> Result<Vec<Phoneme>, String>;
}
