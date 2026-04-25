use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread;
use std::time::Duration;
use crate::application::event_bus::{Event, EventSender};

pub struct KeyboardListener {
    device_state: DeviceState,
    prev_keys: Vec<Keycode>,
}

impl KeyboardListener {
    pub fn new() -> Self {
        Self {
            device_state: DeviceState::new(),
            prev_keys: Vec::new(),
        }
    }

    // This method will poll for new key events and send them through the provided sender.
    pub fn poll_and_send_events(&mut self, sender: &EventSender) {
        let current_keys = self.device_state.get_keys();

        // Detect new key presses
        for key in &current_keys {
            if !self.prev_keys.contains(key) {
                let _ = sender.send(Event::KeyPress(*key));
            }
        }

        // Detect key releases
        for key in &self.prev_keys {
            if !current_keys.contains(key) {
                let _ = sender.send(Event::KeyRelease(*key));
            }
        }

        self.prev_keys = current_keys;
        thread::sleep(Duration::from_millis(10));
    }

    pub fn get_current_keys(&self) -> Vec<Keycode> {
        self.device_state.get_keys()
    }
}
