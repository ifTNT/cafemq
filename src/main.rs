#![crate_name = "cafemq"]

use num_complex::{Complex, Complex32};
use rand::prelude::*;
use std::f32::consts::PI;

fn main() {
    let context = zmq::Context::new();
    let rx = context.socket(zmq::REP).unwrap();
    let tx = context.socket(zmq::REQ).unwrap();

    assert!(rx.bind("tcp://*:4000").is_ok());
    assert!(tx.connect("tcp://localhost:2000").is_ok());

    let noise = awgn();
    println!("{} {}", noise.0, noise.1);

    loop {
        let mut msg;
        msg = rx.recv_multipart(0).unwrap();
        println!("RX");
        for i in &msg {
            //println!("{}", hex::encode(i));
            tx.send(i, 0).unwrap();
        }
        msg = tx.recv_multipart(0).unwrap();
        println!("TX");
        for i in &msg {
            //println!("{}", hex::encode(i));
            rx.send(i, 0).unwrap();
        }
    }
}

// Generate a sample of additive white gaussian noise
// with standard normal distribution in complex data type.
fn awgn() -> Complex32 {
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

    // Normalize the result such that the power of noise equals to one.
    Complex::new(z0, z1).scale(2f32.sqrt())
}

// Apply additive white gaussian noise to the signal with specific SNR.
// Parameters:
//   singal: The samples of source signal.
//   snr: The signal to noise ratio. Represent ing dB.
fn apply_awgn(signal: Vec<Complex32>, snr: f32) -> Vec<Complex32> {
    // Convert SNR from dB to amplitude
    let snr = f32::powf(10.0, snr / 20.0);

    // Calculate the total energy of signal
    let energy: f32 = signal.iter().fold(0f32, |sum, val| sum + val.norm_sqr());

    // Lenght of samples. Cast to f32 for the future process
    let N = signal.len() as f32;

    // Average power = Total energy / Length of samples
    let signal_power: f32 = energy / N;

    // Amplitude of each awgn sample = Power of noise / Numbers of sample
    // = (Power of signal / SNR) / Numbers of sample
    let noise_factor: f32 = signal_power / snr / N;

    // Return a vector containing samples
    // that each sample equels to signal+noise.
    signal
        .iter()
        .map(|x| x + awgn().scale(noise_factor))
        .collect()
}
