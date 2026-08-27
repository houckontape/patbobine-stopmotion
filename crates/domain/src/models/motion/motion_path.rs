use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::motion_axis::MotionAxis;
use super::motion_keyframe::MotionKeyframe;
use super::motion_curve::MotionCurve;

/// Trajectoire d’un axe.
///
/// Compatible avec MotionRig.
/// Un rig peut contenir plusieurs MotionPath.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionPath {
    pub id: Uuid,
    pub axis_name: String,
    pub keyframes: Vec<MotionKeyframe>,
    pub curve: MotionCurve,
}

impl MotionPath {
    pub fn new(axis_name: impl Into<String>, curve: MotionCurve) -> Self {
        Self {
            id: Uuid::new_v4(),
            axis_name: axis_name.into(),
            keyframes: Vec::new(),
            curve,
        }
    }

    pub fn add_keyframe(&mut self, keyframe: MotionKeyframe) {
        self.keyframes.push(keyframe);
        self.keyframes.sort_by_key(|k| k.frame_index);
    }
}
