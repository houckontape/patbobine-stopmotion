/// Port métier : lecture audio.
///
/// L’adapter gère la lecture réelle.
pub trait AudioPlaybackPort {
    fn play(&self) -> Result<(), String>;
    fn pause(&self) -> Result<(), String>;
    fn seek(&self, timestamp_ms: u64) -> Result<(), String>;
}
