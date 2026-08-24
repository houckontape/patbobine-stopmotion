use serde::{Serialize, Deserialize};

/// Métadonnées associées à une frame.
///
/// Inspiré des données disponibles dans Canon EDSDK / Nikon SDK :
/// - ISO
/// - shutter speed
/// - aperture
/// - white balance
/// - color temperature
/// - orientation
///
/// Ce modèle est pur domaine.
/// Les valeurs sont normalisées pour être indépendantes du constructeur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameMetadata {
    /// Sensibilité ISO (ex : 100, 200, 800…)
    pub iso: Option<u32>,

    /// Vitesse d’obturation (ex : 1/50 → 0.02)
    pub shutter_speed: Option<f32>,

    /// Ouverture (ex : f/2.8 → 2.8)
    pub aperture: Option<f32>,

    /// Balance des blancs (ex : "Auto", "Daylight", "Tungsten")
    pub white_balance: Option<String>,

    /// Température de couleur (Kelvin)
    pub color_temperature: Option<u32>,

    /// Orientation (ex : "Landscape", "Portrait")
    pub orientation: Option<String>,
}

impl FrameMetadata {
    pub fn empty() -> Self {
        Self {
            iso: None,
            shutter_speed: None,
            aperture: None,
            white_balance: None,
            color_temperature: None,
            orientation: None,
        }
    }
}
