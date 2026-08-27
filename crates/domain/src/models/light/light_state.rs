use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Représente l’état lumineux d’un rig à un instant.
///
/// Ce modèle est pur domaine.
/// Il ne connaît pas DMX, ArtNet, ni le matériel.
///
/// Chaque entrée du HashMap représente un canal abstrait.
/// Exemple :
/// {
///     "KeyLight": 0.8,
///     "FillLight": 0.4,
///     "BackLight": 1.0
/// }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightState {
    /// Identifiant unique de l’état.
    pub id: Uuid,

    /// Intensité par canal (0.0 → 1.0).
    pub channels: std::collections::HashMap<String, f32>,
}

impl LightState {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            channels: std::collections::HashMap::new(),
        }
    }

    /// Définit l’intensité d’un canal.
    pub fn set_channel(&mut self, name: impl Into<String>, intensity: f32) {
        self.channels.insert(name.into(), intensity.clamp(0.0, 1.0));
    }

    /// Récupère l’intensité d’un canal.
    pub fn get_channel(&self, name: &str) -> Option<f32> {
        self.channels.get(name).copied()
    }
}
