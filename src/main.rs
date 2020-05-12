#![crate_name = "cafemq"]

extern crate cafemq;

use cafemq::awgn;
use num_complex::{Complex, Complex32};

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
        //println!("Received request");
        //println!("{}", hex::encode(&dummy));
        rx.send(&dummy, 0).unwrap();

        samples = rx.recv_multipart(0).unwrap();
        for raw_samples in &samples {
            // Bytes to complex
            let samples: Vec<Complex32> = (0..raw_samples.len())
                .step_by(8)
                .map(|i| bytes2complex(&raw_samples[i..i + 8].to_vec()))
                .collect();

            // [TODO] Process samples;
            let snr: f32 = 0f32;
            let samples = awgn::apply_awgn(&samples, snr);

            // Complex to bytes
            let raw_samples: Vec<Vec<u8>> = samples
                .iter()
                .map(|sample| complex2bytes(&sample))
                .collect();

            // Flatten the byte array
            let raw_samples: Vec<u8> = (0..raw_samples.len() * 8)
                .map(|i| raw_samples[i / 8][i % 8])
                .collect();
            tx.send(&raw_samples, 0).unwrap();
            //println!("{} samples responsed", raw_samples.len());
        }
    }
}

fn bytes2complex(raw: &Vec<u8>) -> Complex32 {
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

fn complex2bytes(c: &Complex32) -> Vec<u8> {
    let re = c.re.to_ne_bytes().to_vec();
    let im = c.im.to_ne_bytes().to_vec();

    let mut rt_val = re;
    rt_val.extend(im);
    rt_val
}
