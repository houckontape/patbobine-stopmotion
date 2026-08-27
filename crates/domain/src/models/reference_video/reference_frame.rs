use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Représente une frame de la vidéo de référence.
///
/// Ce modèle est pur domaine :
/// - il ne contient pas l’image réelle
/// - il ne connaît pas le fichier vidéo
/// - il ne connaît pas le lecteur vidéo
///
/// Il sert à synchroniser la vidéo avec la timeline stop‑motion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceFrame {
    /// Identifiant unique de la frame de référence.
    pub id: Uuid,

    /// Index de la frame dans la vidéo de référence.
    pub index: u32,

    /// Timestamp en millisecondes dans la vidéo.
    pub timestamp_ms: u64,

    /// Chemin logique de l’image extraite (thumbnail ou frame).
    /// L’adapter vidéo se chargera de générer cette image.
    pub logical_image_path: String,
}

impl ReferenceFrame {
    pub fn new(
        index: u32,
        timestamp_ms: u64,
        logical_image_path: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            index,
            timestamp_ms,
            logical_image_path: logical_image_path.into(),
        }
    }
}
