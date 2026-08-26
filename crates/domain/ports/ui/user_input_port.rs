/// Port métier : actions utilisateur.
///
/// Permet de transmettre l’intention métier.
pub trait UserInputPort {
    fn on_capture(&self);
    fn on_play(&self);
    fn on_pause(&self);
    fn on_export(&self);
}
