use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::scene::Scene;
use super::project_settings::ProjectSettings;

/// Modèle principal représentant un projet stop‑motion.
///
/// Un projet contient :
/// - des scènes
/// - des paramètres (frame rate, résolution…)
///
/// Ce modèle est pur domaine.
/// Il ne connaît pas :
/// - la caméra
/// - le stockage
/// - l’UI
///
/// Ces éléments sont gérés via les ports/adapters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Identifiant unique du projet.
    pub id: Uuid,

    /// Nom du projet (ex : "Pat & Bobine — Episode 1").
    pub name: String,

    /// Liste des scènes du projet.
    pub scenes: Vec<Scene>,

    /// Paramètres généraux du projet.
    pub settings: ProjectSettings,
}

impl Project {
    /// Crée un projet vide avec des paramètres donnés.
    pub fn new(name: impl Into<String>, settings: ProjectSettings) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            scenes: Vec::new(),
            settings,
        }
    }

    /// Ajoute une scène au projet.
    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    /// Calcule le nombre total de frames du projet.
    pub fn total_frames(&self) -> u32 {
        self.scenes.iter().map(|s| s.total_frames()).sum()
    }
}
