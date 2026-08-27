use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::light_state::LightState;

/// Une keyframe lumineuse.
///
/// Elle représente l’état du rig à une frame donnée.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightKeyframe {
    /// Frame de la timeline.
    pub frame_index: u32,

    /// État lumineux à cette frame.
    pub state: LightState,
}

impl LightKeyframe {
    pub fn new(frame_index: u32, state: LightState) -> Self {
        Self { frame_index, state }
    }
}

/// Une séquence lumineuse.
///
/// Inspirée de Dragonframe Lighting Programs.
///
/// Contient :
/// - des keyframes
/// - une interpolation (future extension)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightSequence {
    /// Identifiant unique de la séquence.
    pub id: Uuid,

    /// Nom de la séquence (ex : "Sunrise", "Flicker", "Fade In").
    pub name: String,

    /// Keyframes lumineuses.
    pub keyframes: Vec<LightKeyframe>,
}

impl LightSequence {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            keyframes: Vec::new(),
        }
    }

    /// Ajoute une keyframe.
    pub fn add_keyframe(&mut self, keyframe: LightKeyframe) {
        self.keyframes.push(keyframe);
        self.keyframes.sort_by_key(|k| k.frame_index);
    }

    /// Récupère l’état à une frame donnée.
    pub fn state_at(&self, frame_index: u32) -> Option<&LightState> {
        self.keyframes
            .iter()
            .find(|k| k.frame_index == frame_index)
            .map(|k| &k.state)
    }
}
