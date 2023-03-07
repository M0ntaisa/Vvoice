extern crate cpal;

use cpal::traits::{DeviceTrait, HostTrait};
use std::sync::mpsc::{channel, Receiver};
use std::thread;

fn main() {
    // Get the default host and event loop
    let host = cpal::default_host();
    let event_loop = host.new_event_loop();

    // Get the default input device
    let device = host
        .default_input_device()
        .expect("Failed to get default input device");

    // Get the format of the default input device
    let config = device.default_input_config().unwrap();

    // Create a vector to store the captured audio data
    let mut audio_data = Vec::new();

    // Create a channel for the push-to-talk input
    let (tx, rx): (std::sync::mpsc::Sender<bool>, Receiver<bool>) = channel();

    // Create a callback that writes the input stream to the audio_data vector
    let callback = move |data: &[i16], _: &cpal::InputCallbackInfo| {
        if let Ok(push_to_talk) = rx.try_recv() {
            if push_to_talk {
                audio_data.extend_from_slice(data);
            }
        }
    };

    // Start the audio stream and run the event loop
    let stream_id = event_loop
        .build_input_stream(&device, &config.into(), callback, ())
        .unwrap();
    event_loop.play_stream(stream_id).unwrap();
}
