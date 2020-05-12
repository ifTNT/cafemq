#![crate_name = "cafemq"]

extern crate cafemq;

use cafemq::awgn;
use cafemq::binary_complex;

// For multi-threading
use std::thread;

// For command-line arguments processing
use clap::{load_yaml, App};

struct Channel {
    input_socket_uri: String,
    output_socket_uri: String,
    snr: f32,
}

fn main() {
    // Lodes the configuration of clap
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    // Read SNR from commandline argument
    let snr = matches
        .value_of("snr")
        .unwrap()
        .parse::<f32>()
        .expect("Could't parse SNR.");

    // Check whether number of input socket equals to output socket
    assert_eq!(
        matches.occurrences_of("input"),
        matches.occurrences_of("output"),
        "Number of input socket and output socket mismatch"
    );

    println!("CafeMQ is now running...");

    // Read the socket arguments
    let in_sockets: Vec<&str> = matches
        .values_of("input")
        .expect("Input socket not specified")
        .collect();
    let out_sockets: Vec<&str> = matches
        .values_of("output")
        .expect("Output socket not specified")
        .collect();

    // Create threads to handle each channel
    let mut child_threads = vec![];
    for i in 0..in_sockets.len() {

        // The structure that contains the channel information
        let ch = Channel {
            input_socket_uri: in_sockets[i].to_string(),
            output_socket_uri: out_sockets[i].to_string(),
            snr: snr,
        };

        // Print the informations
        println!("Channel #{}", i);
        println!("\tRX socket: {}", &(ch.input_socket_uri));
        println!("\tTX socket: {}", &(ch.output_socket_uri));
        println!("\tChannel model: Additive White Gaussian Noise Channel");
        println!("\tSNR: {}", ch.snr);

        // Spawn new child thread
        child_threads.push((i, thread::spawn(move || channel_thread(i, ch))));
    }

    // Wait for all of the child threads to finish.
    for (i, child) in child_threads {
        match child.join() {
            Ok(_) => println!("Channel #{} terminated.", i),
            Err(_) => println!("Channel #{} terminated with error.", i),
        }
    }
}

fn channel_thread(nth_channel: usize, ch: Channel) {

    // Initialize the socket object
    let context = zmq::Context::new();
    let rx = context.socket(zmq::REQ).unwrap();
    let tx = context.socket(zmq::REP).unwrap();

    // Connect and bind the sockets
    assert!(
        rx.connect(&ch.input_socket_uri).is_ok(),
        "[Ch#{}] Failed to connect {}",
        nth_channel,
        &ch.input_socket_uri
    );
    assert!(
        tx.bind(&ch.output_socket_uri).is_ok(),
        "[Ch#{}] Failed to bind {}",
        nth_channel,
        &ch.output_socket_uri
    );

    // Channel successfully created
    println!("Channel #{} running...", nth_channel);

    // Request from tx
    let mut dummy;
    let mut samples;
    loop {
        // Forward request from tx to rx.
        dummy = tx.recv_bytes(0).unwrap();
        rx.send(&dummy, 0).unwrap();
        
        // Recive the samples from rx.
        samples = rx.recv_multipart(0).unwrap();
        for raw_samples in &samples {
            // Convert samples from bytes to complex.
            let samples = binary_complex::bytes2complex(&raw_samples);

            // Process samples;
            let samples = awgn::apply_awgn(&samples, ch.snr);

            // Convert samples from complex to bytes.
            let modified_samples = binary_complex::complex2bytes(&samples);

            // Transmit processed samples.
            tx.send(&modified_samples, 0).unwrap();
        }
    }
}