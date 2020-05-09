#![crate_name = "cafemq"]

use rand;
use rand_distr::{Distribution, Normal, NormalError};

fn main() {
    let context = zmq::Context::new();
    let rx = context.socket(zmq::REP).unwrap();
    let tx = context.socket(zmq::REQ).unwrap();

    assert!(rx.bind("tcp://*:4000").is_ok());
    assert!(tx.connect("tcp://localhost:2000").is_ok());

    let noise = awgn(20.0).unwrap();
    println!("{}", noise);

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

fn awgn(snr: f64) -> Result<f64, NormalError> {
    let snr = f64::powf(10.0, snr / 20.0); //Convert SNR from dB to amplitude
    let normal = Normal::new(0.0, snr).unwrap();
    let val: f64 = normal.sample(&mut rand::thread_rng());
    Ok(val)
}
