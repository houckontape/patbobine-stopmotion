use serde::{Serialize, Deserialize};

/// Annotation libre dans l’X-Sheet.
///
/// Peut représenter :
/// - une note de mise en scène
/// - un repère de timing
/// - une indication sonore
/// - un commentaire technique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XSheetAnnotation {
    /// Frame concernée.
    pub frame_index: u32,

    /// Texte de l’annotation.
    pub text: String,
}

impl XSheetAnnotation {
    pub fn new(frame_index: u32, text: impl Into<String>) -> Self {
        Self {
            frame_index,
            text: text.into(),
        }
    }
}
