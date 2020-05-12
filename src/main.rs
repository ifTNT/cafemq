#![crate_name = "cafemq"]

extern crate cafemq;

use cafemq::awgn;
use cafemq::binary_complex;

use clap::{load_yaml, App};

fn main() {
    // Lodes the configuration of clap
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let context = zmq::Context::new();
    let rx = context.socket(zmq::REQ).unwrap();
    let tx = context.socket(zmq::REP).unwrap();

    let input_socket_uri = matches
        .value_of("input")
        .expect("Input socket not specified");
    let output_socket_uri = matches
        .value_of("output")
        .expect("Output socket not specified");
    let snr = matches
        .value_of("snr")
        .unwrap()
        .parse::<f32>()
        .expect("Could't parse SNR.");

    assert!(rx.connect(&input_socket_uri).is_ok());
    assert!(tx.bind(&output_socket_uri).is_ok());

    println!("CafeMQ is now running...");
    println!("RX socket: {}", &input_socket_uri);
    println!("TX socket: {}", &output_socket_uri);
    println!("Channel model: Additive White Gaussian Noise Channel");
    println!("SNR: {}", snr);

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
            let samples = awgn::apply_awgn(&samples, snr);

            // Convert samples from complex to bytes.
            let modified_samples = binary_complex::complex2bytes(&samples);
            // Transmit processed samples.
            tx.send(&modified_samples, 0).unwrap();
        }
    }
}
