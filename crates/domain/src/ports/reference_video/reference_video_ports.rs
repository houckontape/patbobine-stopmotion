use crate::models::reference_video::{ReferenceVideo, ReferenceFrame};

pub trait ReferenceVideoPort {
    fn load_video(&self, path: &str) -> Result<ReferenceVideo, String>;
    fn extract_frame(&self, video: &ReferenceVideo, index: u32) -> Result<ReferenceFrame, String>;
}
