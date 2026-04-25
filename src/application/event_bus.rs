use device_query::Keycode;

pub enum Event {
    KeyPress(Keycode),
    KeyRelease(Keycode),
}

pub type EventSender = std::sync::mpsc::Sender<Event>;
pub type EventReceiver = std::sync::mpsc::Receiver<Event>;

pub fn create_event_channel() -> (EventSender, EventReceiver) {
    std::sync::mpsc::channel()
}
