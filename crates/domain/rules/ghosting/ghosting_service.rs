use crate::domain::models::ghosting::{GhostingSettings, GhostFrame};
use crate::domain::models::timeline::Frame;

/// Service métier du ghosting.
///
/// Calcule les ghost frames à partir des frames de la timeline.
/// Ne connaît pas l’UI.
/// Ne connaît pas le rendu graphique.
pub struct GhostingService;

impl GhostingService {
    pub fn compute_ghost_frames(
        &self,
        settings: &GhostingSettings,
        frames: &[Frame],
        current_index: usize,
    ) -> Vec<GhostFrame> {
        if !settings.enabled {
            return Vec::new();
        }

        let mut ghosts = Vec::new();

        for offset in 1..=settings.range {
            let intensity = settings.intensity / offset as f32;

            if matches!(settings.mode, crate::domain::models::ghosting::GhostingMode::Previous | crate::domain::models::ghosting::GhostingMode::Both) {
                if current_index >= offset {
                    let f = &frames[current_index - offset];
                    ghosts.push(GhostFrame::new(
                        f.id,
                        -(offset as i32),
                        intensity,
                        "logical/path/to/frame",
                    ));
                }
            }

            if matches!(settings.mode, crate::domain::models::ghosting::GhostingMode::Next | crate::domain::models::ghosting::GhostingMode::Both) {
                if current_index + offset < frames.len() {
                    let f = &frames[current_index + offset];
                    ghosts.push(GhostFrame::new(
                        f.id,
                        offset as i32,
                        intensity,
                        "logical/path/to/frame",
                    ));
                }
            }
        }

        ghosts
    }
}
