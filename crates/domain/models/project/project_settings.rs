use serde::{Serialize, Deserialize};

/// Configuration générale d’un projet stop‑motion.
///
/// Ce modèle est pur (domaine), sans technologie.
/// Il sert à définir les paramètres de base utilisés par :
/// - la capture (frame rate)
/// - l’export (résolution, aspect ratio)
/// - la création automatique de scènes
///
/// Il est sérialisable pour être stocké dans un repository
/// (fichier JSON, SQLite, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    /// Nombre d’images par seconde (12, 24…)
    /// Impacte la lecture dans la timeline et l’export.
    pub frame_rate: u32,

    /// Largeur de la résolution de travail.
    /// Ne force pas la caméra : c’est une intention de projet.
    pub resolution_width: u32,

    /// Hauteur de la résolution de travail.
    pub resolution_height: u32,

    /// Rapport d’image (ex : "16:9", "4:3").
    /// Utilisé pour les masques d’aspect dans l’UI.
    pub aspect_ratio: String,

    /// Nom par défaut de la première scène créée.
    pub default_scene_name: String,
}

impl Default for ProjectSettings {
    /// Valeurs par défaut raisonnables pour un projet HD.
    fn default() -> Self {
        Self {
            frame_rate: 24,
            resolution_width: 1920,
            resolution_height: 1080,
            aspect_ratio: "16:9".into(),
            default_scene_name: "Scene 01".into(),
        }
    }
}
