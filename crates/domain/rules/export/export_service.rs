use crate::domain::models::export::{ExportSettings, ExportResult};

pub struct ExportService;

impl ExportService {
    pub fn validate_settings(&self, settings: &ExportSettings) -> bool {
        settings.width > 0 &&
            settings.height > 0 &&
            settings.frame_rate > 0 &&
            !settings.output_path.is_empty()
    }
}
