use crate::domain::models::export::{ExportSettings, ExportResult};

pub trait VideoExportPort {
    fn export_video(&self, settings: &ExportSettings) -> Result<ExportResult, String>;
}
