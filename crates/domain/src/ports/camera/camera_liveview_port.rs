/// Port métier : live-view.
///
/// L’adapter fournit une image live-view (JPEG, PNG, raw bytes…)
pub trait CameraLiveViewPort {
    fn get_liveview_frame(&self) -> Result<Vec<u8>, String>;
}
