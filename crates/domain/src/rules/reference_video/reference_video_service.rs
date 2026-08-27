use crate::models::reference_video::{ReferenceVideo, ReferenceFrame};

pub struct ReferenceVideoService;

impl ReferenceVideoService {
    pub fn frame_at_timestamp<'a>(&self, video: &'a ReferenceVideo, timestamp_ms: u64) -> Option<&'a ReferenceFrame> {
        video.frames.iter().find(|f| f.timestamp_ms >= timestamp_ms)
    }
}
