use domain::ports::ui::user_input_port::UserInputPort;
use domain::ports::ui::user_notification_port::UserNotificationPort;

pub struct ConsoleUiAdapter {}

impl ConsoleUiAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl UserNotificationPort for ConsoleUiAdapter {
    fn info(&self, message: &str) {
        println!("[INFO] {}", message);
    }
    fn warn(&self, message: &str) {
        println!("[WARN] {}", message);
    }
    fn error(&self, message: &str) {
        println!("[ERROR] {}", message);
    }
}

impl UserInputPort for ConsoleUiAdapter {
    fn on_capture(&self) {
        println!("Action: Capture");
    }
    fn on_play(&self) {
        println!("Action: Play");
    }
    fn on_pause(&self) {
        println!("Action: Pause");
    }
    fn on_export(&self) {
        println!("Action: Export");
    }
}
