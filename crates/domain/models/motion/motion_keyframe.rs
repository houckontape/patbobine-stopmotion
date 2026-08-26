use serde::{Serialize, Deserialize};

/// Une keyframe de motion control.
///
/// Elle représente :
/// - une position d’axe
/// - à un moment donné (frame index)
///
/// Ce modèle est pur domaine.
/// Il ne connaît pas la timeline réelle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionKeyframe {
    /// Frame de la timeline où s’applique la keyframe.
    pub frame_index: u32,

    /// Position de l’axe à cette frame.
    pub position: f32,
}

impl MotionKeyframe {
    pub fn new(frame_index: u32, position: f32) -> Self {
        Self {
            frame_index,
            position,
        }
    }
}
