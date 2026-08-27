use serde::{Serialize, Deserialize};

/// Snapshot des réglages caméra au moment de la capture.
///
/// Ce modèle permet de :
/// - comparer les réglages demandés vs obtenus
/// - détecter les incohérences
/// - afficher les réglages dans l’UI
///
/// Il est volontairement proche de `FrameMetadata`,
/// mais représente les réglages *intentionnels* ou *effectifs*
/// selon le port caméra.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CameraSettingsSnapshot {
    /// ISO demandé ou obtenu.
    pub iso: Option<u32>,

    /// Vitesse d’obturation.
    pub shutter_speed: Option<f32>,

    /// Ouverture.
    pub aperture: Option<f32>,

    /// Mode de balance des blancs.
    pub white_balance_mode: Option<String>,

    /// Température de couleur.
    pub color_temperature: Option<u32>,
}

impl CameraSettingsSnapshot {
    pub fn empty() -> Self {
        Self {
            iso: None,
            shutter_speed: None,
            aperture: None,
            white_balance_mode: None,
            color_temperature: None,
        }
    }
}
