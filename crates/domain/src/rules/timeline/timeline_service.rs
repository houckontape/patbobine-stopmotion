use crate::models::timeline::{Timeline, Frame};

pub struct TimelineService;

impl TimelineService {
    pub fn add_frame(&self, timeline: &mut Timeline) {
        timeline.add_frame();
    }

    pub fn delete_frame(&self, timeline: &mut Timeline, frame_id: uuid::Uuid) {
        timeline.delete_frame(frame_id);
    }

    pub fn reorder_frame(&self, timeline: &mut Timeline, frame_id: uuid::Uuid, new_index: usize) {
        timeline.move_frame(frame_id, new_index);
    }

    pub fn hide_frame(&self, timeline: &mut Timeline, frame_id: uuid::Uuid) {
        timeline.hide_frame(frame_id);
    }
}
