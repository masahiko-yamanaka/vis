pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use crate::infrastructure::audio::sound_generator::SoundGenerator;
use crate::application::service::music_service::MusicService;
use crate::presentation::gui::app::EguiApp;
use crate::infrastructure::input::keyboard_listener::KeyboardListener; 
use crate::application::event_bus; 

use std::sync::Arc;
use std::thread; 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sound_generator = SoundGenerator::new()?;
    let music_service = Arc::new(MusicService::new(sound_generator));

    // Create the event channel for communication between keyboard listener and GUI
    let (event_sender, event_receiver) = event_bus::create_event_channel();

    // Spawn a new thread for the keyboard listener
    let keyboard_sender_clone = event_sender.clone(); // Clone sender for the new thread
    thread::spawn(move || {
        let mut keyboard_listener = KeyboardListener::new();
        loop {
            keyboard_listener.poll_and_send_events(&keyboard_sender_clone);
            // Optionally, add a small sleep here to reduce CPU usage if polling is very frequent
            // thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Piano Keyboard GUI",
        native_options,
        Box::new(move |_cc| { 
            Ok(Box::new(EguiApp::new(music_service.clone(), event_receiver)))
        }),
    );

    Ok(())
}

