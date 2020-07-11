<p align="center">
<img src="https://raw.githubusercontent.com/ivosequeros/gaze/master/docs/title.png" height="120"></p>
<p align="center"><b>Library + Client for gaze.rs</b></p>

---

### 1. Main structure

On this project there are 3 interesting folders. `src` contains the source code of the program, `macros` contains a macro to make publishing and subscribing easier and `library` contains a black-box library that can be used to connect to Gaze.

As a library user, only the `src` folder is interesting. It contains 2 example of library use: a producer and a subscriber that are run concurrently in `src/main.rs`.

The library folder contains a folder with benchmarkings (see `library/benches`) and the source code of the library (see `library/src`). The main file of the library is `lib.rs`. It provides a Gaze object that can be instantiated and can: connect to Gaze, publish and subscribe.

### 2. Directory sturucture

```
.
├── library
│   ├── benches           # Contains benchmarkings used
│   ├── src
│   │   ├── command.rs    # Network protocol commands
│   │   ├── lib.rs        # 
│   │   ├── message.rs    # Message trait for macro use
│   │   ├── numbers.rs    # Encoding/decoding functions for numbers
│   │   ├── protocol.rs   # Read/Write protocol implementation
│   │   └── reader.rs     # Reads from the stream concurrently and multiplexes messages
├── src
│   ├── main.rs           # Spawns a producer and a subscriber
│   ├── producer.rs       # Produces messages in a loop
│   └── subscriber.rs     # Subscribes to messages and listens using a channel
```

### 3. How to run
#### Raw
To run this project, make sure you have [Rust](https://rustup.rs/) installed. Clone the repo and run `cargo run`.

#### With Docker
Make sure you have [Docker](https://docs.docker.com/get-docker/) installed. Run `docker build .` on the root of this project to build a Docker image. Once the image has been build, run `docker run <image-name>`.
