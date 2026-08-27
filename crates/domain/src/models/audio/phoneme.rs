/// Représente un phonème dans la piste audio.
///
/// Inspiré de Dragonframe :
/// - "A", "E", "I", "O", "U"
/// - "M", "F", "L"
/// - "Rest"
///
/// Un phonème est associé à un timestamp.
#[derive(Debug, Clone)]
pub struct Phoneme {
    /// Code du phonème (ex : "A", "O", "M").
    pub code: String,

    /// Timestamp en millisecondes dans la piste audio.
    pub timestamp_ms: u64,
}

impl Phoneme {
    pub fn new(code: impl Into<String>, timestamp_ms: u64) -> Self {
        Self {
            code: code.into(),
            timestamp_ms,
        }
    }
}
