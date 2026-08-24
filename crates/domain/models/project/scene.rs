use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::shot::Shot;

/// Une scène regroupe plusieurs plans (shots).
///
/// Ce modèle représente la structure narrative.
/// La timeline réelle (frame par frame) est gérée ailleurs.
///
/// Une scène peut :
/// - contenir plusieurs shots
/// - calculer son total de frames
/// - être sérialisée pour stockage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    /// Identifiant unique de la scène.
    pub id: Uuid,

    /// Nom de la scène (ex : "Scene 01").
    pub name: String,

    /// Liste des plans qui composent la scène.
    pub shots: Vec<Shot>,
}

impl Scene {
    /// Crée une scène vide.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            shots: Vec::new(),
        }
    }

    /// Ajoute un plan à la scène.
    pub fn add_shot(&mut self, shot: Shot) {
        self.shots.push(shot);
    }

    /// Calcule le nombre total de frames de la scène.
    /// Utile pour l’UI, l’export, et la timeline globale.
    pub fn total_frames(&self) -> u32 {
        self.shots.iter().map(|s| s.frame_count).sum()
    }
}
