#![crate_name = "cafemq"]

extern crate cafemq;

use cafemq::awgn;
use cafemq::binary_complex;

fn main() {
    let context = zmq::Context::new();
    let rx = context.socket(zmq::REQ).unwrap();
    let tx = context.socket(zmq::REP).unwrap();

    assert!(rx.connect("tcp://localhost:2000").is_ok());
    assert!(tx.bind("tcp://*:4000").is_ok());

    // Request from tx
    let mut dummy;
    let mut samples;
    loop {
        // Forward request from tx to rx.
        dummy = tx.recv_bytes(0).unwrap();
        rx.send(&dummy, 0).unwrap();

        samples = rx.recv_multipart(0).unwrap();
        for raw_samples in &samples {

            // Convert samples from bytes to complex.
            let samples = binary_complex::bytes2complex(&raw_samples);

            // Process samples;
            let snr: f32 = 0f32;
            let samples = awgn::apply_awgn(&samples, snr);

            // Convert samples from complex to bytes.
            let modified_samples = binary_complex::complex2bytes(&samples);
            
            // Transmit processed samples.
            tx.send(&modified_samples, 0).unwrap();
        }
    }
}
