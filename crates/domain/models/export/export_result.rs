use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Résultat d’un export.
///
/// Ce modèle est pur domaine.
/// Il permet de :
/// - informer l’UI
/// - informer les services
/// - stocker l’historique des exports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    /// Identifiant unique de l’export.
    pub id: Uuid,

    /// Chemin logique du fichier exporté.
    pub output_path: String,

    /// Succès ou échec.
    pub success: bool,

    /// Message d’erreur éventuel.
    pub error_message: Option<String>,

    /// Durée de l’export en millisecondes.
    pub duration_ms: u64,
}

impl ExportResult {
    pub fn success(output_path: impl Into<String>, duration_ms: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            output_path: output_path.into(),
            success: true,
            error_message: None,
            duration_ms,
        }
    }

    pub fn failure(output_path: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            output_path: output_path.into(),
            success: false,
            error_message: Some(error.into()),
            duration_ms: 0,
        }
    }
}
