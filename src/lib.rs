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
      // The number of samples
      const N: usize = 1000000;
      // Generate a signal with average power euqals to one.
      let signal = vec![Complex::new(1f32, 0f32); N];

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

pub mod binary_complex {

  use num_complex::{Complex, Complex32};

  // Convert bytes to complex. The input contains multiple sample.
  pub fn bytes2complex(raw_samples: &Vec<u8>) -> Vec<Complex32> {
    (0..raw_samples.len())
      .step_by(8)
      .map(|i| _bytes2complex(&raw_samples[i..i + 8].to_vec()))
      .collect()
  }

  // Convert complex to bytes. The input contains multiple sample.
  pub fn complex2bytes(samples: &Vec<Complex32>) -> Vec<u8> {
    let raw_samples: Vec<Vec<u8>> = samples
      .iter()
      .map(|sample| _complex2bytes(&sample))
      .collect();

    // Flatten the byte array
    (0..raw_samples.len() * 8)
      .map(|i| raw_samples[i / 8][i % 8])
      .collect()
  }

  // Convert two 32-bits long floating number to complex.
  fn _bytes2complex(raw: &Vec<u8>) -> Complex32 {
    // Prepare byte array
    let mut re = [0u8; 4];
    let mut im = [0u8; 4];

    // Convert from slice to byte array
    re.copy_from_slice(&raw[0..4]);
    im.copy_from_slice(&raw[4..8]);

    // Convert from byte array to f32
    let re = f32::from_ne_bytes(re);
    let im = f32::from_ne_bytes(im);

    // Make the complex
    Complex::new(re, im)
  }

  // Convert one complex to two 32-bits long floating number.
  fn _complex2bytes(c: &Complex32) -> Vec<u8> {
    // Convert the real part to bytes. As well as the imaginary part.
    let re = c.re.to_ne_bytes().to_vec();
    let im = c.im.to_ne_bytes().to_vec();

    // Combine the real part and imaginary part.
    let mut rt_val = re;
    rt_val.extend(im);
    rt_val
  }
  #[cfg(test)]
  mod tests {
    use super::*;
    use rand::prelude::*;
    #[test]
    fn inverse_function() {

      // Numbers of samples
      const N: u32 = 1000;

      // Random number generator.
      let mut rng = rand::thread_rng();

      // Generate a complex vector for testing.
      let test_val: Vec<Complex32> = (0..N).map(
        |_| Complex::new(rng.gen(), rng.gen())
      ).collect();

      let bytes = complex2bytes(&test_val);

      let result_val = bytes2complex(&bytes);

      assert_eq!(&test_val, &result_val);
    }
  }
}
