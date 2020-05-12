# CaféMQ

[![Build Status](https://travis-ci.com/ifTNT/cafemq.svg?token=LxKCBVXqDBaiHcSwp4Uc&branch=master)](https://travis-ci.com/ifTNT/cafemq)
[![GPLv3 license](https://img.shields.io/badge/License-LGPLv3-blue.svg)](https://opensource.org/licenses/lgpl-3.0.html)

## Introduction

[srsLTE](https://github.com/srsLTE/srsLTE) provides a virtual RF front-end based on [ZeroMQ](https://zeromq.org/) for convenient development. In short, you can deploy your own LTE EPC, eNB and UE without buying any additional hardware equipments. The method has massive protential in researching and developing.  
  
ZeroMQ is used here as an ideal physical channel to carry all of the baseband samples from Tx to Rx. However, the ideal channel is not suitable to perform any experiment of error-correction due the error is never occured in the ideal channel. Thus, we introduced the CaféMQ (or CafeMQ for convenience). CafeMQ simulates multiple noisy channel to provide a well controled and tunable RF enviorment for the researchers, students, and other hackers.

## Features

- Compatible with srsLTE.
- Tunable additive white Gaussian noise(AWGN) channel model with specificed SNR.
- Hight throughput.
- Written in pure Rust.
- Support AVX2 instruction set. [TODO]

## Architecture
Digging into the implementation of ZeroMQ RF-frontend, srsLTE uses the REQ-and-REP pattern from ZeroMQ by default. In that pattern, a finite-state-machine is maintaind both side during communications to ensure that one response coorsponding to one request. CafeMQ follow this pattern either. What CafeMQ really do is some kind of man-in-the-middle attack. As shown in the following figure, CafeMQ simply extract the baseband samples from ZeroMQ, apply channel model to them, and then transmit to the original destination.
<p align="center">
  <img src="https://github.com/ifTNT/cafemq/raw/master/docs/media/Architecture.png" alt="Architecture of CafeMQ">
</p>

## Installation

CafeMQ is written in rust. Plese refer to [the installation guide of rust](https://www.rust-lang.org/tools/install) for more information.  
 After installation rust and cargo, we need to install the dependency using the following command:

- For Arch Linux User:
  `sudo pacman -S zeromq`

- For Ubuntu User:
  `sudo apt-get install libzmq3-dev`

Now, you can build CafeMQ from the source.

```
git clone https://github.com/ifTNT/cafemq.git
cd cafemq
cargo build --release
```

The fresh binary file is lied in `target/release/cafemq`.  
You can install it to your system via
```
cargo install --path .
```

## Usage

First, refer to [srsLTE](https://github.com/srsLTE/srsLTE) and [ZeroMQ Applaction note](https://docs.srslte.com/en/latest/app_notes/source/zeromq/source/index.html) to install the srsLTE and other dependency.

The following example assume you had installed srsLTE release 20.04.

Launch CafeMQ:

```
cafemq -i "tcp://localhost:2000" -o "tcp://*:4000"
```

Create a new namespace called "ue1":

```
sudo ip netns add ue1
```

Initialize srsepc:

```
sudo srsepc
```

Run the srsenb using the folling command:

```
srsenb --rf.device_name=zmq --rf.device_args="fail_on_disconnect=true,tx_port=tcp://*:2000,rx_port=tcp://localhost:2001,id=enb,base_srate=23.04e6" --expert.nof_phy_threads=1
```

Last, launch srsue

```
sudo ./srsue/src/srsue --rf.device_name=zmq --rf.device_args="tx_port=tcp://*:2001,rx_port=tcp://localhost:4000,id=ue,base_srate=23.04e6" --gw.netns=ue1 --phy.nof_phy_threads=1
```

Vertify the downlink and uplink.

```
ping 172.16.0.2
sudo ip netns exec ue1 ping 172.16.0.1
```

## Result
Use srsgui to observe the signal processed by cafeMQ. We can get the following images:  
- SNR = 20dB ![SNR = 20dB](https://github.com/ifTNT/cafemq/raw/master/docs/media/after_awgn_snr_20dB.png)  
- SNR = 10dB ![SNR = 10dB](https://github.com/ifTNT/cafemq/raw/master/docs/media/after_awgn_snr_10dB.png)  
- SNR = 0dB ![SNR = 0dB](https://github.com/ifTNT/cafemq/raw/master/docs/media/after_awgn_snr_0dB.png)  
- SNR = -10dB ![SNR = -10dB](https://github.com/ifTNT/cafemq/raw/master/docs/media/after_awgn_snr_-10dB.png)  

## Performance
Run `cargo bench` to obtain the benchmark result.  
The following result was tested on Intel Core i5-8250U with Arch Linux.

The criterias are:
- awgn_apply_speed: The speed that applying additive white gaussian noises to a series of samples.
- awgn_random_speed: The speed that generating a additive white gaussian noise.
- bytes2complex: The speed that converting a vector of bytes to a series of complex numbers.
- complex2bytes: The speed that encode a series of complex numbers to bytes.

```
Running target/release/deps/awgn-2b13d644a468bdaa

running 2 tests
test awgn_apply_speed  ... bench:      30,918 ns/iter (+/- 1,143) = 264 MB/s
test awgn_random_speed ... bench:          27 ns/iter (+/- 2) = 296 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured

Running target/release/deps/binary_complex-43ce7fec59481dcd

running 2 tests
test bytes2complex ... bench:      22,556 ns/iter (+/- 2,461) = 354 MB/s
test complex2bytes ... bench:      70,508 ns/iter (+/- 4,554) = 113 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured
```

Notice that all of the throughput is multiply by eight because the lenght of every sample is eight bytes.  
Thus, if we convert the unit from bytes-per-second to samples-per-second, the result will be as follow.  

- awgn_apply_speed: 33 MS/s
- awgn_random_speed: 37 MS/s
- bytes2complex: 44 MS/s
- complex2bytes: 14 MS/s

## Contributer

ifTNT \< happyjohn369 at gmail.com \>
