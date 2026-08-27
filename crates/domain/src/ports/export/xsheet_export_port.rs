use crate::models::export::{ExportSettings, ExportResult};

pub trait XSheetExportPort {
    fn export_xsheet(&self, settings: &ExportSettings) -> Result<ExportResult, String>;
}
