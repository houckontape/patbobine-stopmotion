use domain::models::models::frame::CapturedImage;
use domain::ports::ports::storage::frame_repository::FrameRepository;

pub struct MockStorageAdapter;

impl FrameRepository for MockStorageAdapter {
    fn save_frame(&self, frame: &CapturedImage) -> Result<(), String> {
        // Mock implementation
        println!("Saving frame: {:?}", frame.id);
        Ok(())
    }
    
    fn load_frame(&self, id: uuid::Uuid) -> Result<CapturedImage, String> {
        // Mock implementation
        Err("Not implemented".to_string())
    }
}
