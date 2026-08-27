use crate::models::motion::{MotionRig, MotionPath, MotionKeyframe};

pub struct MotionService;

impl MotionService {
    pub fn add_keyframe(&self, rig: &mut MotionRig, axis_name: &str, frame_index: u32, position: f32) {
        if let Some(path) = rig.paths.iter_mut().find(|p| p.axis_name == axis_name) {
            path.add_keyframe(MotionKeyframe::new(frame_index, position));
        }
    }
}
