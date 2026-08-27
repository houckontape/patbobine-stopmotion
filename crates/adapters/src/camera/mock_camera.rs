use domain::models::models::frame::{CapturedImage, FrameMetadata, CameraSettingsSnapshot};
use domain::ports::ports::camera::camera_capture_port::CameraCapturePort;

pub struct MockCameraAdapter;

impl CameraCapturePort for MockCameraAdapter {
    fn capture(&self) -> Result<(
        CapturedImage, 
        FrameMetadata, 
        CameraSettingsSnapshot
    ), String> {
        // Mock implementation
        Ok((
            CapturedImage::new("mock/path/0001.jpg", 0),
            FrameMetadata::default(), // Note: Need to check if FrameMetadata has Default
            CameraSettingsSnapshot::default(), // Note: Need to check if CameraSettingsSnapshot has Default
        ))
    }
}
