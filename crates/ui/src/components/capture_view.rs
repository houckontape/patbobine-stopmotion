use domain::ports::ui::user_input_port::UserInputPort;
use std::rc::Rc;

pub struct CaptureView<T: UserInputPort> {
    user_input_port: Rc<T>,
}

impl<T: UserInputPort> CaptureView<T> {
    pub fn new(user_input_port: Rc<T>) -> Self {
        Self { user_input_port }
    }

    pub fn trigger_capture(&self) {
        self.user_input_port.on_capture();
    }
}
