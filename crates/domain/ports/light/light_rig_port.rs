use crate::domain::models::light::{LightRig, LightState};

pub trait LightRigPort {
    fn read_state(&self) -> Result<LightState, String>;
    fn apply_state(&self, state: &LightState) -> Result<(), String>;
    fn execute_sequence(&self, rig: &LightRig) -> Result<(), String>;
}
