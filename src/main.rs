use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f64::consts::PI;

struct SinWave {
    hz: f64,
    t: u64,
}

impl SinWave {
    fn new(hz: f64) -> Self {
        SinWave { hz: hz, t: 0 }
    }
}

impl Iterator for SinWave {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let amplitude = (2.0 * PI * self.hz * (self.t as f64) / 44000.0).sin();
        self.t += 1;
        Some(amplitude)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello world!");
    let sin_wave = SinWave::new(440.0);

    let mut sin_wave_iter = sin_wave.into_iter();

    for _ in 1..100 {
        let amplitude = sin_wave_iter.next().unwrap_or(0.0);
        println!("{}", amplitude);
    }

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Did not find default output device");
    let config = device.default_output_config().unwrap();
    dbg!(&config);

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let stream_config: cpal::StreamConfig = config.into();

    let stream = device.build_output_stream(
        &stream_config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            write_data(data, 2, &mut sin_wave_iter)
        },
        err_fn,
        None,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));
    Ok(())
}

fn write_data(output: &mut [f32], channels: usize, next_sample: &mut dyn Iterator<Item = f64>) {
    for frame in output.chunks_mut(channels) {
        let sample = next_sample.next().unwrap();
        for s in frame.iter_mut() {
            *s = sample as f32;
        }
    }
}
