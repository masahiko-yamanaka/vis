use eframe::egui;
use std::sync::Arc;
use std::collections::HashSet;
use crate::application::service::music_service::MusicService;
use device_query::Keycode;
use crate::application::event_bus::{Event, EventReceiver};

pub struct EguiApp {
    music_service: Arc<MusicService>,
    event_receiver: EventReceiver,
    pressed_keys: HashSet<Keycode>,
}

impl EguiApp {
    pub fn new(music_service: Arc<MusicService>, event_receiver: EventReceiver) -> Self {
        Self {
            music_service,
            event_receiver,
            pressed_keys: HashSet::new(),
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process events from the keyboard listener
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                Event::KeyPress(keycode) => {
                    if self.pressed_keys.insert(keycode) { // Only play if key was not already pressed
                        self.music_service.handle_key_press(keycode);
                    }
                }
                Event::KeyRelease(keycode) => {
                    self.pressed_keys.remove(&keycode);
                }
            }
        }

        // Request repaint to update the UI with new key states
        ctx.request_repaint();
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) { 
        egui::CentralPanel::default().show_inside(ui, |ui_inside| {
            ui_inside.heading("Piano Keyboard");

            let keyboard_rect = ui_inside.available_rect_before_wrap();
            let key_width = keyboard_rect.width() / 14.0; 
            let key_height = keyboard_rect.height();

            // Define keycodes for white keys (C-D-E-F-G-A-B for two octaves)
            let white_keys_map = vec![
                (Keycode::Z, "C"), (Keycode::X, "D"), (Keycode::C, "E"), (Keycode::V, "F"),
                (Keycode::B, "G"), (Keycode::N, "A"), (Keycode::M, "B"),
                (Keycode::Key0, "C"), (Keycode::Key1, "D"), (Keycode::Key2, "E"), (Keycode::Key3, "F"),
            ];

            let mut current_x = keyboard_rect.left();

            for (keycode, label) in white_keys_map.iter() {
                let key_color = if self.pressed_keys.contains(keycode) {
                    egui::Color32::GRAY // Key is pressed, make it gray
                } else {
                    egui::Color32::WHITE // Key is not pressed, keep it white
                };

                let rect = egui::Rect::from_min_size(egui::pos2(current_x, keyboard_rect.top()), egui::vec2(key_width, key_height));
                let response = ui_inside.put(rect, egui::Button::new(*label).fill(key_color));

                if response.clicked() {
                    // Handle GUI clicks: simulate a press and then a release
                    self.music_service.handle_key_press(*keycode);
                    // A more complete solution would simulate release after a short delay
                }

                current_x += key_width;
            }

            // TODO: Add black keys

        });
    }
    
}
