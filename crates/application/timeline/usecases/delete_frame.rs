use domain::ports::storage::timeline_repository::TimelineRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteFrameUseCase {
    timeline_repository: Arc<dyn TimelineRepository + Send + Sync>,
}

impl DeleteFrameUseCase {
    pub fn new(timeline_repository: Arc<dyn TimelineRepository + Send + Sync>) -> Self {
        Self {
            timeline_repository,
        }
    }

    pub fn execute(&self, timeline_id: Uuid, frame_id: Uuid) -> Result<(), String> {
        let mut timeline = self.timeline_repository.load(timeline_id)?;
        timeline.delete_frame(frame_id);
        self.timeline_repository.save(&timeline)?;
        Ok(())
    }
}
