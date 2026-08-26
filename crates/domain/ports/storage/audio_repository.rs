use crate::domain::models::audio::AudioTrack;

pub trait AudioRepository {
    fn save(&self, audio: &AudioTrack) -> Result<(), String>;
    fn load(&self, id: uuid::Uuid) -> Result<AudioTrack, String>;
}
