use serde::{Serialize, Deserialize};

use super::export_format::ExportFormat;

/// Paramètres d’export.
///
/// Ce modèle est pur domaine.
/// Il ne connaît pas FFmpeg, ni le système de fichiers.
///
/// Il représente l’intention de l’utilisateur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSettings {
    /// Format d’export (image sequence, vidéo, GIF…)
    pub format: ExportFormat,

    /// Résolution de sortie.
    pub width: u32,
    pub height: u32,

    /// Framerate de sortie.
    pub frame_rate: u32,

    /// Chemin logique de sortie (ex : "exports/final.mp4").
    pub output_path: String,

    /// Qualité (0 → 100) pour les formats compressés.
    pub quality: Option<u8>,

    /// Inclure l’audio dans l’export vidéo.
    pub include_audio: bool,

    /// Exporter les frames cachées (hidden frames).
    pub include_hidden_frames: bool,
}

impl ExportSettings {
    pub fn new(
        format: ExportFormat,
        width: u32,
        height: u32,
        frame_rate: u32,
        output_path: impl Into<String>,
    ) -> Self {
        Self {
            format,
            width,
            height,
            frame_rate,
            output_path: output_path.into(),
            quality: None,
            include_audio: true,
            include_hidden_frames: false,
        }
    }
}
