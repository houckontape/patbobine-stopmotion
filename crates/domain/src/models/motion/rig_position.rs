use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Position complète d’un rig.
///
/// Contient la position de chaque axe.
/// Ce modèle est pur domaine.
/// Il ne connaît pas le matériel réel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RigPosition {
    /// Identifiant unique de la position.
    pub id: Uuid,

    /// Nom de la position (ex : "Start", "Middle", "End").
    pub name: String,

    /// Positions des axes, par nom d’axe.
    /// Exemple :
    /// {
    ///     "Pan": 12.5,
    ///     "Tilt": -3.0,
    ///     "Slide": 120.0
    /// }
    pub axis_positions: std::collections::HashMap<String, f32>,
}

impl RigPosition {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            axis_positions: std::collections::HashMap::new(),
        }
    }

    /// Définit la position d’un axe.
    pub fn set_axis_position(&mut self, axis_name: impl Into<String>, position: f32) {
        self.axis_positions.insert(axis_name.into(), position);
    }

    /// Récupère la position d’un axe.
    pub fn get_axis_position(&self, axis_name: &str) -> Option<f32> {
        self.axis_positions.get(axis_name).copied()
    }
}
