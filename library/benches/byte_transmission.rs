#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::time::SystemTime;
use std::net::TcpStream;
use std::io::Write;
use rand::{Rng, thread_rng};
use std::iter;
use rand::distributions::Alphanumeric;

fn transmit_byte(stream: &mut TcpStream) {
    stream.write(&[1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8]).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let ip = "127.0.0.1";

    let mut stream: TcpStream = match TcpStream::connect(format!("{}:3000", ip)) {
        Ok(stream) => {
            println!("Connected to server on {}:3000", ip);
            stream
        },
        Err(error) => {
            println!("Cannot connect to server: {}", error);
            return;
        }
    };

    c.bench_function("transmit 1 byte", |b| b.iter(|| transmit_byte(&mut stream)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);