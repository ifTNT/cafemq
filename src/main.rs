#![crate_name = "cafemq"]

extern crate cafemq;

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
        println!("Received request");
        println!("{}", hex::encode(&dummy));
        rx.send(&dummy, 0).unwrap();

        samples = rx.recv_multipart(0).unwrap();
        for i in &samples {
            let samples = i;
            //println!("{}", hex::encode(i));
            tx.send(&samples, 0).unwrap();
        }
        println!("{} samples responsed", 0);
    }
}
