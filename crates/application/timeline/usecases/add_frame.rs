use domain::ports::storage::timeline_repository::TimelineRepository;
use domain::models::timeline::timeline::Timeline;
use std::sync::Arc;
use uuid::Uuid;

pub struct AddFrameUseCase {
    timeline_repository: Arc<dyn TimelineRepository + Send + Sync>,
}

impl AddFrameUseCase {
    pub fn new(timeline_repository: Arc<dyn TimelineRepository + Send + Sync>) -> Self {
        Self {
            timeline_repository,
        }
    }

    pub fn execute(&self, timeline_id: Uuid) -> Result<(), String> {
        let mut timeline = self.timeline_repository.load(timeline_id)?;
        timeline.add_frame();
        self.timeline_repository.save(&timeline)?;
        Ok(())
    }
}
