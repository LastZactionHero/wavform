use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f64::consts::PI;

mod frequencies;
use frequencies::Note;

struct SinWave {
    hz: f64,
    t: u64,
}

impl SinWave {
    fn new(hz: f64) -> Self {
        SinWave { hz, t: 0 }
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

struct CombinedWave {
    waves: Vec<SinWave>,
}

impl Iterator for CombinedWave {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut amplitude = 0.0;
        for wave in &mut self.waves {
            let wave_iter = wave.into_iter();
            amplitude += wave_iter.next().unwrap_or(0.0);
        }
        return Some(amplitude / self.waves.len() as f64);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sin_wave_a440 = SinWave::new(frequencies::frequency(Note::A4));
    let sin_wave_e329 = SinWave::new(frequencies::frequency(Note::E4));
    let sin_wave_cs277 = SinWave::new(frequencies::frequency(Note::Csharp4));

    let combined_wave = CombinedWave {
        waves: vec![sin_wave_a440, sin_wave_e329, sin_wave_cs277],
    };

    let mut combined_wave_iter = combined_wave.into_iter();

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
            write_data(data, 2, &mut combined_wave_iter)
        },
        err_fn,
        None,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(3000));
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
