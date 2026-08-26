use crate::domain::models::xsheet::{XSheet, XSheetCell, XSheetAnnotation};

pub struct XSheetService;

impl XSheetService {
    pub fn set_cell(&self, xsheet: &mut XSheet, column_name: &str, frame_index: u32, value: String) {
        if let Some(column) = xsheet.columns.iter_mut().find(|c| c.name == column_name) {
            if let Some(cell) = column.cells.iter_mut().find(|c| c.frame_index == frame_index) {
                cell.value = Some(value);
            } else {
                let mut cell = XSheetCell::new(frame_index);
                cell.value = Some(value);
                column.cells.push(cell);
            }
        }
    }

    pub fn add_annotation(&self, xsheet: &mut XSheet, frame_index: u32, text: String) {
        xsheet.annotations.push(XSheetAnnotation::new(frame_index, text));
    }
}
