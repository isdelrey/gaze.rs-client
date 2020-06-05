FROM rustlang/rust:nightly

WORKDIR /usr/src

COPY . .

RUN cargo build --release

CMD ["/usr/src/target/release/gazeclient"]