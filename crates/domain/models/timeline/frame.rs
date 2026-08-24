use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Représente une frame dans la timeline.
/// Inspiré des concepts Dragonframe :
/// - hidden frames
/// - held frames
/// - deleted frames (delete bin)
/// - markers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    /// Identifiant unique de la frame.
    pub id: Uuid,

    /// Index logique dans la timeline.
    /// Ce n’est pas l’index physique du fichier image.
    pub index: u32,

    /// Indique si la frame est cachée (hidden).
    /// Dragonframe : “Hidden Frames won’t appear in playback”
    pub hidden: bool,

    /// Indique si la frame est un “hold” (répétition).
    /// Dragonframe : “Held Frames: This means the same frame is repeated.”
    pub hold: bool,

    /// Indique si la frame est dans le delete bin.
    /// Dragonframe : “Deleted frames are moved to the Delete Bin.”
    pub deleted: bool,

    /// Marqueurs éventuels (nommés).
    pub markers: Vec<String>,
}

impl Frame {
    pub fn new(index: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            index,
            hidden: false,
            hold: false,
            deleted: false,
            markers: Vec::new(),
        }
    }
}
