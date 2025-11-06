FROM rust:latest

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

WORKDIR /app

RUN git clone https://github.com/intentee/poet.git .

RUN cargo build --release

RUN mv target/release/poet /usr/local/bin/poet

ENTRYPOINT ["poet"]
