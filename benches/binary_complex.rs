extern crate cafemq;

#[macro_use]
extern crate bencher;

use bencher::Bencher;
use cafemq::binary_complex;
use num_complex::{Complex, Complex32};
use rand::prelude::*;

fn bytes2complex(bench: &mut Bencher) {
  // Numbers of samples
  const N: u32 = 1000;

  // Random number generator.
  let mut rng = rand::thread_rng();

  // Generate a complex vector for testing.
  let test_val: Vec<Complex32> = (0..N).map(|_| Complex::new(rng.gen(), rng.gen())).collect();

  let bytes = binary_complex::complex2bytes(&test_val);

  bench.iter(|| {
    binary_complex::bytes2complex(&bytes);
  });

  // 8 Bytes per sample.
  bench.bytes = (N * 8) as u64;
}

fn complex2bytes(bench: &mut Bencher) {
  // Numbers of samples
  const N: u32 = 1000;

  // Random number generator.
  let mut rng = rand::thread_rng();

  // Generate a complex vector for testing.
  let test_val: Vec<Complex32> = (0..N).map(|_| Complex::new(rng.gen(), rng.gen())).collect();

  bench.iter(|| {
    binary_complex::complex2bytes(&test_val);
  });

  // 8 Bytes per sample.
  bench.bytes = (N * 8) as u64;
}

benchmark_group!(benches, bytes2complex, complex2bytes);
benchmark_main!(benches);
