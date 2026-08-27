use crate::models::xsheet::XSheet;

pub trait XSheetRepository {
    fn save(&self, xsheet: &XSheet) -> Result<(), String>;
    fn load(&self, id: uuid::Uuid) -> Result<XSheet, String>;
}
