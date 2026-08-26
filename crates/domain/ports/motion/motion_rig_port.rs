use crate::domain::models::motion::{MotionRig, RigPosition};

pub trait MotionRigPort {
    fn read_position(&self) -> Result<RigPosition, String>;
    fn apply_position(&self, position: &RigPosition) -> Result<(), String>;
    fn execute_rig(&self, rig: &MotionRig) -> Result<(), String>;
}
