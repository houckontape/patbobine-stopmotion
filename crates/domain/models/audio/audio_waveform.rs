use serde::{Serialize, Deserialize};

/// Représente la waveform d’une piste audio.
///
/// Ce modèle est pur domaine.
/// Il contient une version simplifiée de la waveform :
/// - amplitude normalisée (0.0 → 1.0)
/// - échantillons réduits pour l’UI
///
/// Le calcul réel (FFT, downsampling) est fait dans un adapter audio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioWaveform {
    /// Amplitudes normalisées.
    /// Exemple : 1000 valeurs pour représenter toute la piste.
    pub samples: Vec<f32>,
}

impl AudioWaveform {
    pub fn new(samples: Vec<f32>) -> Self {
        Self { samples }
    }
}
