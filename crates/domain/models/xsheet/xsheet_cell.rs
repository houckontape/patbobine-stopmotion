use serde::{Serialize, Deserialize};

/// Une cellule de l’X-Sheet.
///
/// Une cellule représente une information sur une frame donnée.
/// Elle peut contenir :
/// - un phonème (ex : "A", "O", "M") pour le lip-sync
/// - une action (ex : "blink", "jump")
/// - une annotation libre
///
/// Ce modèle est pur domaine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XSheetCell {
    /// Index de la frame dans la timeline.
    pub frame_index: u32,

    /// Contenu textuel de la cellule.
    /// Dragonframe utilise des codes courts : "A", "O", "M", "BLINK", etc.
    pub value: Option<String>,
}

impl XSheetCell {
    pub fn new(frame_index: u32) -> Self {
        Self {
            frame_index,
            value: None,
        }
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = Some(value.into());
    }
}
