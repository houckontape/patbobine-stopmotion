use serde::{Serialize, Deserialize};

/// Formats d’export disponibles.
///
/// Inspiré des logiciels stop-motion :
/// - Séquence d’images
/// - Vidéo (MP4, MOV)
/// - GIF
/// - Export X-Sheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    /// Export en séquence d’images (ex : PNG, JPG).
    ImageSequence {
        extension: String, // "png", "jpg"
    },

    /// Export vidéo (ex : MP4, MOV).
    Video {
        codec: String,     // "h264", "prores"
        container: String, // "mp4", "mov"
    },

    /// Export GIF animé.
    Gif,

    /// Export X-Sheet (CSV, JSON…)
    XSheet {
        extension: String, // "csv", "json"
    },
}
