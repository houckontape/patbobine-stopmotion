/// Port métier : notifications utilisateur.
///
/// L’adapter UI (Tauri, Web, CLI…) implémente ce trait.
pub trait UserNotificationPort {
    fn info(&self, message: &str);
    fn warn(&self, message: &str);
    fn error(&self, message: &str);
}
