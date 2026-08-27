use domain::ports::camera::camera_capture_port::CameraCapturePort;
use domain::ports::storage::frame_repository::FrameRepository;
use domain::models::frame::captured_image::CapturedImage;
use domain::models::frame::frame_metadata::FrameMetadata;
use domain::models::frame::camera_settings_snapshot::CameraSettingsSnapshot;
use std::sync::Arc;

pub struct CaptureFrameUseCase {
    camera_port: Arc<dyn CameraCapturePort + Send + Sync>,
    frame_repository: Arc<dyn FrameRepository + Send + Sync>,
}

impl CaptureFrameUseCase {
    pub fn new(
        camera_port: Arc<dyn CameraCapturePort + Send + Sync>,
        frame_repository: Arc<dyn FrameRepository + Send + Sync>,
    ) -> Self {
        Self {
            camera_port,
            frame_repository,
        }
    }

    pub fn execute(&self) -> Result<(), String> {
        let (image, metadata, settings) = self.camera_port.capture()?;
        self.frame_repository.save_frame(&image)?;
        Ok(())
    }
}
