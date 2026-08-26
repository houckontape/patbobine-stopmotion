use crate::domain::models::light::{LightRig, LightSequence, LightKeyframe, LightState};

pub struct LightService;

impl LightService {
    pub fn apply_sequence(&self, rig: &mut LightRig, sequence_name: &str, frame_index: u32) {
        if let Some(seq) = rig.sequence_named(sequence_name) {
            if let Some(state) = seq.state_at(frame_index) {
                rig.apply_state(state.clone());
            }
        }
    }
}
