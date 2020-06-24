#[macro_use]
extern crate criterion;

use criterion::Criterion;
use gazeclient::gaze::time::SmallestReadableString;
use std::time::SystemTime;
use std::net::TcpStream;
use std::io::Write;
use rand::{Rng, thread_rng};
use std::iter;
use std::rc::Rc;
use rand::distributions::Alphanumeric;
use avro_rs::{types::Record, Reader, Writer, Schema};


fn decode(message: Vec<u8>, schema: Rc<Schema>) -> () {
    let reader = Reader::with_schema(&schema, &message[..]).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let raw_schema = r#"
       {
            "type": "record",
            "namespace": "my.company",
            "name": "user_created",
            "fields": [
                {"name": "name", "type": "string"},
                {"name": "age", "type": "int"}
            ]
        }
    "#;

    let message: Vec<u8> = Vec::new();
    let schema = Schema::parse_str(raw_schema).unwrap();

    let mut writer = Writer::new(&schema, message);
    let mut record = Record::new(writer.schema()).unwrap();
    record.put("name", "Ivo");
    record.put("age", 24);

    writer.append(record).unwrap();
    let message = writer.into_inner().unwrap();

    let schema = Rc::new(schema);

    c.bench_function("decode message", |b| b.iter(|| decode(message.to_vec(), schema.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);