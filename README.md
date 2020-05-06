# CaféMQ
[![Build Status](https://travis-ci.com/ifTNT/cafemq.svg?token=LxKCBVXqDBaiHcSwp4Uc&branch=master)](https://travis-ci.com/ifTNT/cafemq)  

## Introduction
[srsLTE](https://github.com/srsLTE/srsLTE) provided a virtual RF front-end based on [ZeroMQ](https://zeromq.org/) for convenient development. ZeroMQ is used here as an ideal physical channel to carry all of the I/Q data from Tx to Rx. However, the ideal channel is not suitable to perform an experiment of error-correction due the error is never occured in the ideal channel.  
Thus, we introduced the CaféMQ(or CafeMQ for convenience), an I/Q data ralay with tunable noisy channel model based on ZeroMQ.

## Features
- Compatible with srsLTE.
- Tunable noisy channel model.
  - Applying white noise to signal.
  - Mixing I/Q data of different channel to emulate the interference between radio frequency.
  - Multipath effect emulation.
- Hight throughput.

## Installation
CafeMQ is written in rust. Plese refer to [the installation guide of rust](https://www.rust-lang.org/tools/install) for more information.  
After installation rust and cargo, we need to install the dependency using the following command:
- For Arch Linux User:
`sudo pacman -S zeromq`

- For Ubuntu User:
`sudo apt-get install libzmq3-dev`

Now, you can build CafeMQ form the source.

```
git clone https://github.com/ifTNT/cafemq.git
cd cafemq
cargo build --release
```

The fresh binary file is lied in `target/release/cafemq`.

## Usage

## Performance

## Contributer
