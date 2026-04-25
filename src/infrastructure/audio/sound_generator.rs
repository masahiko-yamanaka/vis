use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use cpal::Stream;
use rand; // Import rand crate

use crate::domain::model::value_object::voice::{Voice, SoundType};

pub struct SoundGenerator {
    active_voices: Arc<Mutex<Vec<Voice>>>,
    _stream: Stream,
}

impl SoundGenerator {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No output device");
        let config = device.default_output_config()?;
        let sample_rate_val = config.sample_rate().0 as f32;

        let active_voices: Arc<Mutex<Vec<Voice>>> = Arc::new(Mutex::new(Vec::new()));
        let voices_for_stream = active_voices.clone();

        let stream = device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo|
            {
                let mut voices = voices_for_stream.lock().unwrap();
                for frame in data.chunks_mut(2) {
                    let mut mix = 0.0_f32;
                    for v in voices.iter_mut() {
                        let t = v.time;
                        match v.sound_type {
                            SoundType::Cymbal => {
                                // --- Cymbal Sound: White noise + High-pass filter ---
                                let noise = rand::random::<f32>() * 2.0 - 1.0;
                                let env = (-t * 8.0).exp(); // Cymbal decay
                                mix += noise * env * 0.15;
                            }
                            SoundType::Piano => {
                                // --- Piano Sound: Superposition of inharmonic overtones ---
                                let f = v.freq;
                                let mut s = (t * f * 2.0 * std::f32::consts::PI).sin() * 1.0;
                                s += (t * f * 2.01 * 2.0 * std::f32::consts::PI).sin() * 0.5; // Slightly detuned string
                                s += (t * f * 3.02 * 2.0 * std::f32::consts::PI).sin() * 0.2;
                                s += (t * f * 4.04 * 2.0 * std::f32::consts::PI).sin() * 0.1;
                                let decay = (-t * 1.2).exp();
                                mix += s * decay * 0.1;
                            }
                        }
                        v.time += 1.0 / sample_rate_val;
                    }
                    voices.retain(|v| v.time < 5.0);
                    
                    // Limiter
                    mix = mix.clamp(-0.9, 0.9);
                    frame[0] = mix; frame[1] = mix;
                }
            },
            |err| eprintln!("Error: {}", err),
            None,
        )?;
        stream.play()?;

        Ok(Self {
            active_voices,
            _stream: stream,
        })
    }

    pub fn add_voice(&self, voice: Voice) {
        self.active_voices.lock().unwrap().push(voice);
    }
}
