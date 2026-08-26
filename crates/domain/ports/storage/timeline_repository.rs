use crate::domain::models::timeline::Timeline;

pub trait TimelineRepository {
    fn save(&self, timeline: &Timeline) -> Result<(), String>;
    fn load(&self, id: uuid::Uuid) -> Result<Timeline, String>;
}
