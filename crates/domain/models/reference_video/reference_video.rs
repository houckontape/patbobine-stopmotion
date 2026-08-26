use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::reference_frame::ReferenceFrame;

/// Représente une vidéo de référence importée.
///
/// Inspiré de Dragonframe :
/// - Movie Overlay
/// - Compare Movie to Animation
/// - Frame-by-frame stepping
/// - Scrubbing synchronisé
///
/// Ce modèle est pur domaine.
/// Il ne connaît pas :
/// - le fichier vidéo réel
/// - le lecteur vidéo
/// - le rendu overlay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceVideo {
    /// Identifiant unique de la vidéo de référence.
    pub id: Uuid,

    /// Chemin logique du fichier vidéo (ex : "reference/movie.mp4").
    pub logical_path: String,

    /// Durée totale en millisecondes.
    pub duration_ms: u64,

    /// Fréquence d’images (FPS) de la vidéo.
    pub frame_rate: u32,

    /// Frames extraites de la vidéo.
    /// L’adapter vidéo se chargera de les générer.
    pub frames: Vec<ReferenceFrame>,

    /// Active ou désactive l’overlay vidéo.
    pub overlay_enabled: bool,

    /// Opacité de l’overlay (0.0 → 1.0).
    pub overlay_opacity: f32,
}

impl ReferenceVideo {
    pub fn new(
        logical_path: impl Into<String>,
        duration_ms: u64,
        frame_rate: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            logical_path: logical_path.into(),
            duration_ms,
            frame_rate,
            frames: Vec::new(),
            overlay_enabled: false,
            overlay_opacity: 0.5,
        }
    }

    /// Ajoute une frame de référence.
    pub fn add_frame(&mut self, frame: ReferenceFrame) {
        self.frames.push(frame);
    }

    /// Récupère une frame par index.
    pub fn frame_at(&self, index: u32) -> Option<&ReferenceFrame> {
        self.frames.iter().find(|f| f.index == index)
    }

    /// Active l’overlay vidéo.
    pub fn enable_overlay(&mut self) {
        self.overlay_enabled = true;
    }

    /// Désactive l’overlay vidéo.
    pub fn disable_overlay(&mut self) {
        self.overlay_enabled = false;
    }

    /// Change l’opacité de l’overlay.
    pub fn set_overlay_opacity(&mut self, opacity: f32) {
        self.overlay_opacity = opacity.clamp(0.0, 1.0);
    }
}
