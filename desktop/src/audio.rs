use rodio::{OutputStream, Sink};
use std::{f32::consts::PI, sync::{Arc, Mutex}};

use crate::config::Config;

pub struct AudioManager {
    frequency: f32, 
    duration: f32,
    volume: f32,
    _stream: Arc<Mutex<OutputStream>>,
    sink: Arc<Mutex<Sink>>,
}

impl AudioManager {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        AudioManager {
            frequency: 0.,
            duration: 0.,
            volume: 0.,

            _stream: Arc::new(Mutex::new(stream)),
            sink: Arc::new(Mutex::new(sink)),
        }
    }

    pub fn load_values_from_config(&mut self, config: &Config) {
        self.frequency = config.frequency;
        self.duration = config.duration;
        self.volume = config.volume;
    }

    pub fn play_async_beep(&self) {

        let sink = self.sink.clone();
        let frequency = self.frequency;
        let duration = self.duration;
        let volume = self.volume;

        // Spawn a new thread to play the sound
        std::thread::spawn(move || {
            let sink = sink.lock().unwrap();
            sink.set_volume(volume);

            // Generate beep sound as samples
            let samples: Vec<f32> = (0..(44100.0 * duration) as usize)
                .map(|t| (t as f32 * frequency * 2.0 * PI / 44100.0).sin())
                .collect();
        
            // Play the sound
            sink.append(rodio::buffer::SamplesBuffer::new(1, 44100, samples));

            // Ensure the sound plays to completion
            sink.sleep_until_end();
        });
    }
}