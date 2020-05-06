use rand_distr::{Distribution, StandardNormal, NormalError};

fn awgn(snr: f64) -> Result<(), NormalError>{
  snr = f64::pow(10, snr/20); //Convert SNR from dB to amplitude
  let normal = Normal::new(0.0, snr).unwrap();
  let val: f64 = normal.sample(&mut rand::thread_rng());
  println!("{}", val);
}