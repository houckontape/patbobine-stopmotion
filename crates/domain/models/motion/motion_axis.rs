use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Représente un axe de motion control.
///
/// Inspiré des rigs Dragonframe / Kessler / Edelkrone :
/// - Pan
/// - Tilt
/// - Slide
/// - Focus
/// - Zoom
/// - Custom
///
/// Ce modèle est pur domaine.
/// Il ne connaît pas le matériel réel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MotionAxisType {
    Pan,
    Tilt,
    Slide,
    Focus,
    Zoom,
    Custom(String),
}

/// Un axe motorisé.
///
/// Il possède :
/// - un nom
/// - un type
/// - une position actuelle (en unités abstraites)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionAxis {
    /// Identifiant unique de l’axe.
    pub id: Uuid,

    /// Nom de l’axe (ex : "Pan", "Focus").
    pub name: String,

    /// Type de l’axe.
    pub axis_type: MotionAxisType,

    /// Position actuelle (unité abstraite).
    /// L’adapter hardware convertira en degrés / mm / pas moteur.
    pub position: f32,
}

impl MotionAxis {
    pub fn new(name: impl Into<String>, axis_type: MotionAxisType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            axis_type,
            position: 0.0,
        }
    }

    /// Change la position de l’axe.
    pub fn set_position(&mut self, pos: f32) {
        self.position = pos;
    }
}
