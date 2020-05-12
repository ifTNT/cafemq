extern crate cafemq;

#[macro_use]
extern crate bencher;

use cafemq::awgn;
use bencher::Bencher;
use num_complex::Complex;

fn awgn_random_speed(bencher: &mut Bencher) {
  bencher.iter(|| awgn::awgn());

  // 8 Bytes per sample.
  bencher.bytes =  8u64;
}

fn awgn_apply_speed(bench: &mut Bencher) {
  const N: usize = 1024;
  let signal = vec![Complex::new(1f32, 1f32); N];
  bench.iter(|| {
    awgn::apply_awgn(&signal, 30f32);
  });

  // 8 Bytes per sample.
  bench.bytes = (N*8) as u64;
}

benchmark_group!(benches, awgn_random_speed, awgn_apply_speed);
benchmark_main!(benches);
