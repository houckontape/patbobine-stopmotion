use crate::domain::models::frame::CameraSettingsSnapshot;

/// Port métier : réglages caméra.
///
/// Permet de lire ou modifier les réglages.
pub trait CameraSettingsPort {
    fn read_settings(&self) -> Result<CameraSettingsSnapshot, String>;
    fn apply_settings(&self, settings: CameraSettingsSnapshot) -> Result<(), String>;
}
