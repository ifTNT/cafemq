# CaféMQ

[![Build Status](https://travis-ci.com/ifTNT/cafemq.svg?token=LxKCBVXqDBaiHcSwp4Uc&branch=master)](https://travis-ci.com/ifTNT/cafemq)

## Introduction

[srsLTE](https://github.com/srsLTE/srsLTE) provided a virtual RF front-end based on [ZeroMQ](https://zeromq.org/) for convenient development. ZeroMQ is used here as an ideal physical channel to carry all of the I/Q data from Tx to Rx. However, the ideal channel is not suitable to perform an experiment of error-correction due the error is never occured in the ideal channel.

Thus, we introduced the CaféMQ (or CafeMQ for convenience), an I/Q data ralay with tunable noisy channel model based on ZeroMQ.

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

Now, you can build CafeMQ from the source.

```
git clone https://github.com/ifTNT/cafemq.git
cd cafemq
cargo build --release
```

The fresh binary file is lied in `target/release/cafemq`.

## Usage

First, refer to [srsLTE](https://github.com/srsLTE/srsLTE) and [ZeroMQ Applaction note](https://docs.srslte.com/en/latest/app_notes/source/zeromq/source/index.html) to install the srsLTE and other dependency.

The following example assume you had installed srsLTE release 20.04.

Launch CafeMQ:

```
[todo]
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
srsenb --rf.device_name=zmq --rf.device_args="fail_on_disconnect=true,tx_port=tcp://*:2000,rx_port=tcp://localhost:4001,id=enb,base_srate=23.04e6" --expert.nof_phy_threads=1
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

## Performance

## Contributer

ifTNT \< happyjohn369 at gmail.com \>
