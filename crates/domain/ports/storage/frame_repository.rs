use crate::domain::models::frame::CapturedImage;

pub trait FrameRepository {
    fn save_frame(&self, frame: &CapturedImage) -> Result<(), String>;
    fn load_frame(&self, id: uuid::Uuid) -> Result<CapturedImage, String>;
}
