use crate::models::frame::{CapturedImage, FrameMetadata, CameraSettingsSnapshot};

/// Port métier : capture d’image.
///
/// L’adapter (Canon, Nikon, Webcam…) implémente ce trait.
/// Le domaine ne connaît pas la technologie.
pub trait CameraCapturePort {
    /// Capture une image et retourne :
    /// - l’image capturée
    /// - les métadonnées
    /// - le snapshot des réglages caméra
    fn capture(&self) -> Result<(CapturedImage, FrameMetadata, CameraSettingsSnapshot), String>;
}
