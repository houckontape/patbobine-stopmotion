// Simulation des ports pour débloquer la compilation
pub trait CameraCapturePort: Send + Sync {
    fn capture(&self) -> Result<(Vec<u8>, String, String), String>;
}

pub trait FrameRepository: Send + Sync {
    fn save_frame(&self, frame: &[u8]) -> Result<(), String>;
}
