use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::xheet_column::XSheetColumn;
use super::annotation::XSheetAnnotation as Annotation;

/// Modèle principal de l’X-Sheet.
///
/// L’X-Sheet est une grille temporelle utilisée pour :
/// - le lip-sync
/// - le timing
/// - les actions
/// - les repères de scène
/// - les annotations
///
/// Ce modèle est pur domaine.
/// Il ne connaît pas la timeline, mais utilise les mêmes index de frames.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XSheet {
    /// Identifiant unique de l’X-Sheet.
    pub id: Uuid,

    /// Colonnes de l’X-Sheet.
    pub columns: Vec<XSheetColumn>,

    /// Annotations libres.
    pub annotations: Vec<Annotation>,
}

impl XSheet {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            columns: Vec::new(),
            annotations: Vec::new(),
        }
    }

    /// Ajoute une colonne.
    pub fn add_column(&mut self, column: XSheetColumn) {
        self.columns.push(column);
    }

    /// Ajoute une annotation.
    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.push(annotation);
    }

    /// Récupère une colonne par son nom.
    pub fn column_named(&self, name: &str) -> Option<&XSheetColumn> {
        self.columns.iter().find(|c| c.name == name)
    }
}
