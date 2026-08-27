use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Représente un "plan" dans une scène.
///
/// Un shot est une unité narrative :
/// - il contient un nombre de frames capturées
/// - il peut avoir des notes (mise en scène, intentions)
/// - il est identifié par un UUID pour éviter les collisions
///
/// Ce modèle reste volontairement simple.
/// La timeline détaillée est gérée dans `models/timeline`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shot {
    /// Identifiant unique du plan.
    pub id: Uuid,

    /// Nom du plan (ex : "Plan 01", "Close-up Bobine").
    pub name: String,

    /// Nombre total de frames capturées dans ce plan.
    /// Ce n’est pas la timeline : juste un résumé.
    pub frame_count: u32,

    /// Notes facultatives (mise en scène, intentions, remarques).
    pub notes: Option<String>,
}

impl Shot {
    /// Crée un shot vide, prêt à recevoir des frames.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            frame_count: 0,
            notes: None,
        }
    }
}
