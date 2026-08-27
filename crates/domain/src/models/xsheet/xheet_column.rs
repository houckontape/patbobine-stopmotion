use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::xsheet_cell::XSheetCell;

/// Type de colonne.
/// Inspiré de Dragonframe :
/// - Dialogue
/// - Phonèmes
/// - Actions
/// - Caméra
/// - Lumière
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum XSheetColumnType {
    Dialogue,
    Phoneme,
    Action,
    Camera,
    Lighting,
    Custom(String),
}

/// Une colonne de l’X-Sheet.
///
/// Une colonne contient une liste de cellules.
/// Chaque colonne représente une "piste" d’information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XSheetColumn {
    /// Identifiant unique de la colonne.
    pub id: Uuid,

    /// Nom de la colonne.
    pub name: String,

    /// Type de colonne.
    pub column_type: XSheetColumnType,

    /// Cellules de la colonne.
    pub cells: Vec<XSheetCell>,
}

impl XSheetColumn {
    pub fn new(name: impl Into<String>, column_type: XSheetColumnType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            column_type,
            cells: Vec::new(),
        }
    }

    /// Ajoute une cellule à la colonne.
    pub fn add_cell(&mut self, cell: XSheetCell) {
        self.cells.push(cell);
    }

    /// Récupère une cellule par frame.
    pub fn cell_at(&self, frame_index: u32) -> Option<&XSheetCell> {
        self.cells.iter().find(|c| c.frame_index == frame_index)
    }
}
