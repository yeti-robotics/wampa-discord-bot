FROM rust:latest as builder

WORKDIR /usr/src/wampa
RUN git clone https://github.com/samperlmutter/wampa.git  .

RUN cargo build --release

FROM ubuntu:latest

COPY --from=builder /usr/src/wampa/target/release/wampa /usr/local/bin/wampa

CMD ["/usr/local/bin/wampa"]