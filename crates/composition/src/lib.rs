use application::capture::usecases::capture_frame::CaptureFrameUseCase;
use domain::ports::camera::camera_capture_port::CameraCapturePort;
use domain::ports::storage::frame_repository::FrameRepository;
use std::sync::Arc;

pub struct CompositionRoot {
    pub capture_frame_usecase: Arc<CaptureFrameUseCase>,
}

impl CompositionRoot {
    pub fn new(
        camera_port: Arc<dyn CameraCapturePort + Send + Sync>,
        frame_repository: Arc<dyn FrameRepository + Send + Sync>,
    ) -> Self {
        Self {
            capture_frame_usecase: Arc::new(CaptureFrameUseCase::new(camera_port, frame_repository)),
        }
    }
}
