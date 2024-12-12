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

fn main() -> std::io::Result<()> {
    println!("Hello world!");
    let sin_wave = SinWave::new(440.0);

    let mut sin_wave_iter = sin_wave.into_iter();

    for _ in 1..100 {
        let amplitude = sin_wave_iter.next().unwrap_or(0.0);
        println!("{}", amplitude);
    }
    Ok(())
}
