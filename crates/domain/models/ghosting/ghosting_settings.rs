use serde::{Serialize, Deserialize};

/// Mode de ghosting (onion-skin).
/// Inspiré des options Dragonframe :
/// - Previous
/// - Next
/// - Both
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GhostingMode {
    Previous,
    Next,
    Both,
}

/// Paramètres de ghosting.
///
/// Ce modèle est pur domaine.
/// Il ne connaît pas :
/// - la caméra
/// - le rendu graphique
/// - la timeline
///
/// Il représente l’intention de l’utilisateur.
/// Les adapters UI/renderer appliqueront ces paramètres.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostingSettings {
    /// Mode de ghosting (previous / next / both).
    pub mode: GhostingMode,

    /// Nombre de frames à afficher en ghost.
    /// Dragonframe : “Ghost Range”
    pub range: u32,

    /// Intensité du ghost (0.0 → 1.0).
    /// Dragonframe : “Ghost Intensity”
    pub intensity: f32,

    /// Teinte colorée optionnelle (ex : "#FF0000").
    /// Dragonframe : “Ghost Color Tint”
    pub tint_color: Option<String>,

    /// Active ou désactive le ghosting.
    pub enabled: bool,
}

impl Default for GhostingSettings {
    fn default() -> Self {
        Self {
            mode: GhostingMode::Previous,
            range: 1,
            intensity: 0.5,
            tint_color: None,
            enabled: true,
        }
    }
}
