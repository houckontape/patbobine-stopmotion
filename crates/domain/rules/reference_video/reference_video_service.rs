use crate::domain::models::reference_video::{ReferenceVideo, ReferenceFrame};

pub struct ReferenceVideoService;

impl ReferenceVideoService {
    pub fn frame_at_timestamp(&self, video: &ReferenceVideo, timestamp_ms: u64) -> Option<&ReferenceFrame> {
        video.frames.iter().find(|f| f.timestamp_ms >= timestamp_ms)
    }
}
