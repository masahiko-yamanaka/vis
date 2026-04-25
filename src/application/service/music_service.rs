use crate::domain::model::value_object::voice::{Voice, SoundType};
use crate::infrastructure::audio::sound_generator::SoundGenerator;
use device_query::Keycode;

pub struct MusicService {
    sound_generator: SoundGenerator,
}

impl MusicService {
    pub fn new(sound_generator: SoundGenerator) -> Self {
        Self { sound_generator }
    }

    pub fn handle_key_press(&self, key: Keycode) {
        match key {
            // Single notes
            Keycode::Key1 => self.sound_generator.add_voice(Voice { freq: 261.63, time: 0.0, sound_type: SoundType::Piano }),
            Keycode::Key2 => self.sound_generator.add_voice(Voice { freq: 293.66, time: 0.0, sound_type: SoundType::Piano }),
            Keycode::Key3 => self.sound_generator.add_voice(Voice { freq: 329.63, time: 0.0, sound_type: SoundType::Piano }),
            Keycode::Key4 => self.sound_generator.add_voice(Voice { freq: 349.23, time: 0.0, sound_type: SoundType::Piano }),
            Keycode::Key5 => self.sound_generator.add_voice(Voice { freq: 392.00, time: 0.0, sound_type: SoundType::Piano }),
            Keycode::Key6 => self.sound_generator.add_voice(Voice { freq: 440.00, time: 0.0, sound_type: SoundType::Piano }),
            Keycode::Key7 => self.sound_generator.add_voice(Voice { freq: 493.88, time: 0.0, sound_type: SoundType::Piano }),
            Keycode::Key8 => self.sound_generator.add_voice(Voice { freq: 523.25, time: 0.0, sound_type: SoundType::Piano }),
            Keycode::Key9 => self.sound_generator.add_voice(Voice { freq: 587.33, time: 0.0, sound_type: SoundType::Piano }),
            // Chord patterns (add multiple frequencies at once)
            Keycode::V => { // C Major (C, E, G)
                for &f in &[261.63, 329.63, 392.00] {
                    self.sound_generator.add_voice(Voice { freq: f, time: 0.0, sound_type: SoundType::Piano });
                }
            }
            Keycode::B => { // G Major (G, B, D)
                for &f in &[196.00, 246.94, 293.66] {
                    self.sound_generator.add_voice(Voice { freq: f, time: 0.0, sound_type: SoundType::Piano });
                }
            }
            Keycode::N => { // A Minor (A, C, E)
                for &f in &[220.00, 261.63, 329.63] {
                    self.sound_generator.add_voice(Voice { freq: f, time: 0.0, sound_type: SoundType::Piano });
                }
            }
            // Cymbal
            Keycode::Space => self.sound_generator.add_voice(Voice { freq: 0.0, time: 0.0, sound_type: SoundType::Cymbal }),
            _ => {},
        }
    }
}
