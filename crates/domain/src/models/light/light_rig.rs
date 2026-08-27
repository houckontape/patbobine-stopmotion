use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::light_sequence::LightSequence;
use super::light_state::LightState;

/// Représente un rig lumineux.
///
/// Inspiré de Dragonframe :
/// - DMX channels
/// - Lighting Programs
/// - Light Table
///
/// Ce modèle est pur domaine.
/// Il ne connaît pas DMX, ArtNet, ni le matériel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightRig {
    /// Identifiant unique du rig.
    pub id: Uuid,

    /// Nom du rig (ex : "Studio Rig", "Keylight Rig").
    pub name: String,

    /// Liste des canaux disponibles.
    /// Exemple :
    /// ["KeyLight", "FillLight", "BackLight"]
    pub channels: Vec<String>,

    /// Séquences lumineuses associées au rig.
    pub sequences: Vec<LightSequence>,

    /// État lumineux actuel.
    pub current_state: LightState,
}

impl LightRig {
    pub fn new(name: impl Into<String>, channels: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            channels,
            sequences: Vec::new(),
            current_state: LightState::new(),
        }
    }

    /// Ajoute une séquence lumineuse.
    pub fn add_sequence(&mut self, sequence: LightSequence) {
        self.sequences.push(sequence);
    }

    /// Applique un état lumineux au rig.
    pub fn apply_state(&mut self, state: LightState) {
        self.current_state = state;
    }

    /// Récupère une séquence par son nom.
    pub fn sequence_named(&self, name: &str) -> Option<&LightSequence> {
        self.sequences.iter().find(|s| s.name == name)
    }
}
