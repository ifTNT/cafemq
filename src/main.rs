#![crate_name = "cafemq"]

use num_complex::{Complex, Complex32};

extern crate cafemq;

fn main() {
    let context = zmq::Context::new();
    let rx = context.socket(zmq::REP).unwrap();
    let tx = context.socket(zmq::REQ).unwrap();

    assert!(rx.bind("tcp://*:4000").is_ok());
    assert!(tx.connect("tcp://localhost:2000").is_ok());

    let mut sum:Complex32 = Complex::new(0f32,0f32);
    let mut square_sum:f32 = 0.0;
    const n:u64 = 100000;
    for _ in 0..n {
        let sample = cafemq::awgn::awgn();
        sum += sample;
        square_sum += sample.norm_sqr();
    }
    println!("Mu: {}", sum/n as f32);
    println!("Sigma^2: {}", square_sum/n as f32);

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