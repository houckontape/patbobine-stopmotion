/// Mapping entre les frames et les phonèmes.
///
/// Ce modèle est pur domaine.
/// Il permet de synchroniser :
/// - l’X-Sheet
/// - la timeline
/// - l’animation des bouches
///
/// Exemple :
/// frame 12 → "A"
/// frame 13 → "O"
#[derive(Debug, Clone)]
pub struct PhonemeMap {
    /// Liste des phonèmes par frame.
    /// Index = frame_index
    /// Valeur = phonème (ex : "A")
    pub frame_phonemes: Vec<Option<String>>,
}

impl PhonemeMap {
    pub fn new(frame_count: usize) -> Self {
        Self {
            frame_phonemes: vec![None; frame_count],
        }
    }

    pub fn set_phoneme(&mut self, frame_index: usize, phoneme: impl Into<String>) {
        if frame_index < self.frame_phonemes.len() {
            self.frame_phonemes[frame_index] = Some(phoneme.into());
        }
    }

    pub fn get_phoneme(&self, frame_index: usize) -> Option<&String> {
        self.frame_phonemes
            .get(frame_index)
            .and_then(|p| p.as_ref())
    }
}
