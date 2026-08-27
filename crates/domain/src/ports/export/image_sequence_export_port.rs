use crate::models::export::{ExportSettings, ExportResult};

pub trait ImageSequenceExportPort {
    fn export_sequence(&self, settings: &ExportSettings) -> Result<ExportResult, String>;
}
