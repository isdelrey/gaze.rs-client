#[macro_use]
extern crate criterion;

use criterion::Criterion;
use gazeclient::gaze::time::SmallestReadableString;
use std::time::SystemTime;
use std::net::TcpStream;
use std::io::Write;
use rand::{Rng, thread_rng};
use std::iter;
use rand::distributions::Alphanumeric;

fn generate_id() -> Vec<u8> {
     let mut rng = thread_rng();
        let random: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric) )
        .take(4).collect();

        let mut ns = [0u8; 6];
        let ns = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().to_smallest_readable_string(&mut ns);    

        [ns, random.as_bytes()].concat()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("generate id", |b| b.iter(|| generate_id()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);