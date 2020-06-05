#[macro_use]
extern crate criterion;

use std::iter;
use criterion::Criterion;
use criterion::BenchmarkId;
use criterion::Throughput;
use std::time::SystemTime;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::iter::Iterator;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

fn read(stream: &mut TcpStream, m: &mut [u8]) {
    stream.read_exact(m).unwrap();
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

    let mut group = c.benchmark_group("read_benchmark");
    for size in (1..100 as u64).step_by(1) {
        group.throughput(Throughput::Bytes(size));
        group.bench_with_input(BenchmarkId::new("single read", size), &size, |b, &size| {
           let mut m = Vec::with_capacity(size as usize);
           b.iter(|| read(&mut stream, &mut m));
        });
         group.bench_with_input(BenchmarkId::new("read byte by byte", size), &size, |b, &size| {
           let mut m = Vec::with_capacity(1);
           b.iter(|| {
               let mut i = 0;
                while i < size {
                    read(&mut stream, &mut m);
                    i += 1;
                }
            });
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);