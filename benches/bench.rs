extern crate cafemq;

#[macro_use]
extern crate bencher;

use cafemq::awgn;

use bencher::Bencher;

fn benchmark(bencher: &mut Bencher) {
  bencher.iter(|| awgn::awgn());
}

benchmark_group!(benches, benchmark);
benchmark_main!(benches);
