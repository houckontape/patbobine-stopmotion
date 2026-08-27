use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Représente une image capturée par la caméra.
///
/// Ce modèle est pur domaine :
/// - il ne connaît pas le disque
/// - il ne connaît pas la caméra
/// - il ne connaît pas les ports
///
/// Le chemin est un chemin logique (ex : "frames/0001.jpg"),
/// qui sera résolu par un adapter de stockage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedImage {
    /// Identifiant unique de l’image.
    pub id: Uuid,

    /// Chemin logique de l’image dans le projet.
    pub logical_path: String,

    /// Timestamp de capture (millisecondes depuis epoch).
    pub timestamp_ms: u64,

    /// Hash optionnel pour vérifier l’intégrité.
    pub hash: Option<String>,
}

impl CapturedImage {
    pub fn new(logical_path: impl Into<String>, timestamp_ms: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            logical_path: logical_path.into(),
            timestamp_ms,
            hash: None,
        }
    }
}
