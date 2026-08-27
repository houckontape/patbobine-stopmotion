use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::motion_axis::MotionAxis;
use super::motion_path::MotionPath;
use super::rig_position::RigPosition;

/// Représente un rig complet.
///
/// Un rig contient :
/// - plusieurs axes motorisés
/// - des trajectoires (motion paths)
/// - des positions enregistrées
///
/// Ce modèle est pur domaine.
/// Il ne connaît pas le matériel réel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionRig {
    /// Identifiant unique du rig.
    pub id: Uuid,

    /// Nom du rig (ex : "Kessler Crane", "Edelkrone Slider").
    pub name: String,

    /// Axes du rig.
    pub axes: Vec<MotionAxis>,

    /// Trajectoires associées au rig.
    pub paths: Vec<MotionPath>,

    /// Positions enregistrées (presets).
    pub saved_positions: Vec<RigPosition>,
}

impl MotionRig {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            axes: Vec::new(),
            paths: Vec::new(),
            saved_positions: Vec::new(),
        }
    }

    /// Ajoute un axe au rig.
    pub fn add_axis(&mut self, axis: MotionAxis) {
        self.axes.push(axis);
    }

    /// Ajoute une trajectoire.
    pub fn add_path(&mut self, path: MotionPath) {
        self.paths.push(path);
    }

    /// Enregistre une position complète du rig.
    pub fn save_position(&mut self, position: RigPosition) {
        self.saved_positions.push(position);
    }

    /// Récupère une position enregistrée par son nom.
    pub fn position_named(&self, name: &str) -> Option<&RigPosition> {
        self.saved_positions.iter().find(|p| p.name == name)
    }
}
