pub mod awgn {
  use num_complex::{Complex, Complex32};
  use rand::prelude::*;
  use std::f32::consts::PI;

  // Generate a sample of complex additive white gaussian noise
  // with standard normal distribution.
  pub fn awgn() -> Complex32 {
    // Random number generator
    let mut rng = rand::thread_rng();

    // Generate two independent uniform sample in range (f32::EPSILON, 1]
    let mut u1: f32;
    let mut u2: f32;
    loop {
      u1 = rng.gen();
      u2 = rng.gen();
      if u1 > f32::EPSILON && u2 > f32::EPSILON {
        break;
      }
    }

    // Generate two independent standard gaussian sample
    // using Box-Muller transform.
    let z0: f32 = (-2f32 * u1.ln()).sqrt() * (2f32 * PI * u2).cos();
    let z1: f32 = (-2f32 * u1.ln()).sqrt() * (2f32 * PI * u2).sin();

    // Normalize the result such that the varience of noise equals to one.
    Complex::new(z0, z1).scale(1f32 / 2f32.sqrt())
  }

  pub fn calc_power(signal: &Vec<Complex32>) -> f32 {
    // Calculate the total energy of signal
    let energy: f32 = signal.iter().fold(0f32, |acc, x| acc + x.norm_sqr());

    // Lenght of samples. Cast to f32 for the future process
    let n = signal.len() as f32;

    // Average power = Total energy / Length of samples
    energy / n
  }
  // Apply additive white gaussian noise to the signal with specific SNR.
  // Parameters:
  //   singal: The samples of source signal.
  //   snr: The signal to noise ratio. Represent ing dB.
  pub fn apply_awgn(signal: &Vec<Complex32>, snr: f32) -> Vec<Complex32> {
    // Convert SNR from dB to power ratio
    let snr = f32::powf(10.0, snr / 10.0);

    let signal_power = calc_power(&signal);

    // Amplitude of each awgn sample = sqrt(Power of noise)
    // = sqrt(Power of signal / SNR)
    let noise_factor: f32 = (signal_power / snr).sqrt();

    // Return a vector containing samples
    // that each sample equels to signal+noise.
    signal
      .iter()
      .map(|x| x + awgn().scale(noise_factor))
      .collect()
  }
  #[cfg(test)]
  mod tests {
    use super::*;
    use num_complex::Complex;
    #[test]
    fn standard_gaussian() {
      // The number of samples
      const N: u64 = 1000000;

      // Accumulate the samples in order to perform statistics
      let mut sum = Complex::new(0f32, 0f32);
      let mut square_sum: f32 = 0.0;
      for _ in 0..N {
        let sample = awgn();
        sum += sample;
        square_sum += sample.norm_sqr();
      }

      //Calculate the expected value and the varience of samples
      let mu = sum / N as f32;
      let var = square_sum / N as f32;

      // Perform tests
      assert!(
        (mu - Complex::new(0f32, 0f32)).norm() < 10E-2,
        "E[awgn]={}≠0+0i",
        mu
      );
      assert!((var - 1f32).abs() < 1E-2, "Var[awgn]={}≠1", var);
    }

    #[test]
    fn white_noise() {
      // Generate a signal with average power euqals to one.
      let signal = vec![Complex::new(1f32, 0f32); 100];

      // Apply awgn to signal.
      let dirty_signal = apply_awgn(&signal, 0f32);

      // Refine the noise from polluted signal.
      let mut noise = vec![Complex::new(0f32, 0f32); dirty_signal.len()];
      for (i, sample) in dirty_signal.iter().enumerate() {
        noise[i] = sample - signal[i];
      }

      // Calculate the average power of noise.
      let noise_power = calc_power(&noise);

      // By the definition, white noise should have average power equals to one.
      assert!(
        (noise_power - 1f32).abs() < 10E-2,
        "Not white noise. Power of noise={}",
        noise_power
      );
    }

    #[test]
    fn snr() {
      // Random number generator.
      let mut rng = rand::thread_rng();

      // Generate a random signal.
      let signal = (0..10000)
        .map(|_| Complex::new(rng.gen(), rng.gen()))
        .collect();

      // Test SNR  from -30dB to 30dB.
      for snr in (-30..30).step_by(10) {
        let snr: f32 = snr as f32;

        // Apply awgn to the signal.
        let dirty_signal = apply_awgn(&signal, snr);

        // Refine the noise from polluted signal.
        let mut noise = vec![Complex::new(0f32, 0f32); dirty_signal.len()];
        for (i, sample) in dirty_signal.iter().enumerate() {
          noise[i] = sample - signal[i];
        }

        // Caculate the SNR
        let signal_power = calc_power(&signal);
        let noise_power = calc_power(&noise);

        let calculated_snr = 10f32 * (signal_power / noise_power).log10();

        assert!(
          (snr - calculated_snr).abs() < 10E-1,
          "SNR mismatch. target SNR={}dB, caculated SNR={}dB, Power of Signal={}, Power of Noise={}",
          snr,
          calculated_snr,
          signal_power,
          noise_power
        );
      }
    }
  }
}
