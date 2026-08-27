use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Représente une frame fantôme (ghost frame).
///
/// Ce modèle est pur domaine.
/// Il ne contient pas l’image elle-même, mais une référence logique.
///
/// Le rendu graphique (blending, opacity, tint) sera fait dans un adapter UI.
///
/// Une ghost frame peut représenter :
/// - une frame précédente
/// - une frame suivante
/// - une frame interpolée (future IA)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostFrame {
    /// Identifiant de la frame source.
    pub source_frame_id: Uuid,

    /// Décalage par rapport à la frame courante.
    /// ex : -1 = frame précédente, +1 = frame suivante.
    pub offset: i32,

    /// Opacité calculée (0.0 → 1.0).
    /// Dépend de GhostingSettings.intensity et du rang.
    pub opacity: f32,

    /// Chemin logique de l’image source.
    /// Le rendu UI utilisera ce chemin pour afficher l’image fantôme.
    pub logical_image_path: String,
}

impl GhostFrame {
    pub fn new(
        source_frame_id: Uuid,
        offset: i32,
        opacity: f32,
        logical_image_path: impl Into<String>,
    ) -> Self {
        Self {
            source_frame_id,
            offset,
            opacity,
            logical_image_path: logical_image_path.into(),
        }
    }
}
