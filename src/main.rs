#![crate_name = "cafemq"]

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

// Generate two independent additive white standard gaussian noise
fn awgn() -> (f32,f32) {
    
    // Random number generator
    let mut rng = rand::thread_rng();

    // Generate two uniform sample that in (f32::EPSILON, 1]
    let mut u1:f32;
    let mut u2:f32;
    loop{
        u1 = rng.gen();
        u2 = rng.gen();
        if u1>f32::EPSILON && u2>f32::EPSILON {
            break;
        }
    }

    // Use Box-Muller transform to generate two independent standard gaussian sample
    let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
    let z1 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).sin();

    (z0,z1)
}

// Apply additive white gaussian noise to the signal with specific SNR.
fn apply_awgn(signal:Vec<(f32,f32)>, snr:f32) -> Vec<(f32,f32)>{
    
    let snr = f32::powf(10.0, snr / 20.0); //Convert SNR from dB to amplitude

    signal
}