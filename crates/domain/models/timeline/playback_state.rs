use serde::{Serialize, Deserialize};

/// État du playback.
/// Inspiré de Dragonframe :
/// - in point
/// - out point
/// - playhead
/// - looping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackState {
    /// Position du playhead.
    pub playhead: u32,

    /// In point (début du playback).
    /// Dragonframe : “Press I to set the in point.”
    pub in_point: Option<u32>,

    /// Out point (fin du playback).
    /// Dragonframe : “Press O to set the out point.”
    pub out_point: Option<u32>,

    /// Lecture en boucle.
    pub looping: bool,
}

impl PlaybackState {
    pub fn new() -> Self {
        Self {
            playhead: 0,
            in_point: None,
            out_point: None,
            looping: false,
        }
    }
}
