use crate::models::frame::{CapturedImage, FrameMetadata, CameraSettingsSnapshot};

/// Service métier de capture.
///
/// Ce service définit *comment* une capture doit être validée.
/// Il ne connaît pas la caméra.
/// Il ne connaît pas le stockage.
///
/// Le port `CameraCapturePort` fera la capture réelle.
pub struct CaptureService;

impl CaptureService {
    /// Valide une capture.
    ///
    /// Règles métier :
    /// - l’image doit exister (port caméra)
    /// - les métadonnées doivent être cohérentes
    /// - le snapshot doit être complet
    pub fn validate_capture(
        &self,
        image: &CapturedImage,
        metadata: &FrameMetadata,
        snapshot: &CameraSettingsSnapshot,
    ) -> bool {
        // Règle métier : ISO doit être cohérent
        if let (Some(iso_meta), Some(iso_snap)) = (metadata.iso, snapshot.iso) {
            if (iso_meta as i32 - iso_snap as i32).abs() > 200 {
                return false;
            }
        }

        // Règle métier : shutter speed doit être positif
        if let Some(shutter) = metadata.shutter_speed {
            if shutter <= 0.0 {
                return false;
            }
        }

        true
    }
}
