#[macro_use]
extern crate criterion;

use criterion::Criterion;
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

        let ns = (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() % 1000000000) as usize / 1000; 
        let ns_as_string = ns.to_string(); 
        [ns_as_string.as_bytes(), &random.as_bytes()].concat()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("generate num-based id", |b| b.iter(|| generate_id()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);