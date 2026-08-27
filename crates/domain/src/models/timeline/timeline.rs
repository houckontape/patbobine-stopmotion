use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::frame::Frame;
use super::playback_state::PlaybackState;

/// Timeline complète.
/// Inspirée de Dragonframe :
/// - thumbnails
/// - hidden frames
/// - delete bin
/// - reorder
/// - markers
/// - playback range
/// - conform edits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    /// Identifiant unique de la timeline.
    pub id: Uuid,

    /// Liste ordonnée des frames.
    pub frames: Vec<Frame>,

    /// Frames supprimées (delete bin).
    pub delete_bin: Vec<Frame>,

    /// État du playback.
    pub playback: PlaybackState,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            frames: Vec::new(),
            delete_bin: Vec::new(),
            playback: PlaybackState::new(),
        }
    }

    /// Ajoute une frame à la timeline.
    pub fn add_frame(&mut self) {
        let index = self.frames.len() as u32;
        self.frames.push(Frame::new(index));
    }

    /// Cache une frame (hidden).
    pub fn hide_frame(&mut self, frame_id: Uuid) {
        if let Some(f) = self.frames.iter_mut().find(|f| f.id == frame_id) {
            f.hidden = true;
        }
    }

    /// Supprime une frame (delete bin).
    pub fn delete_frame(&mut self, frame_id: Uuid) {
        if let Some(pos) = self.frames.iter().position(|f| f.id == frame_id) {
            let mut frame = self.frames.remove(pos);
            frame.deleted = true;
            self.delete_bin.push(frame);
        }
    }

    /// Restaure une frame depuis le delete bin.
    pub fn restore_frame(&mut self, frame_id: Uuid) {
        if let Some(pos) = self.delete_bin.iter().position(|f| f.id == frame_id) {
            let mut frame = self.delete_bin.remove(pos);
            frame.deleted = false;
            self.frames.insert(frame.index as usize, frame);
        }
    }

    /// Ajoute un marker sur une frame.
    pub fn add_marker(&mut self, frame_id: Uuid, name: impl Into<String>) {
        if let Some(f) = self.frames.iter_mut().find(|f| f.id == frame_id) {
            f.markers.push(name.into());
        }
    }

    /// Déplace une frame (reorder).
    pub fn move_frame(&mut self, frame_id: Uuid, new_index: usize) {
        if let Some(pos) = self.frames.iter().position(|f| f.id == frame_id) {
            let frame = self.frames.remove(pos);
            self.frames.insert(new_index, frame);

            // Recalcule les index
            for (i, f) in self.frames.iter_mut().enumerate() {
                f.index = i as u32;
            }
        }
    }
}
