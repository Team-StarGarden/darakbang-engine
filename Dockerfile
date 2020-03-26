FROM rust as builder

WORKDIR /usr/src/darakbang-engine

COPY Cargo.lock .
COPY Cargo.toml .

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm src/main.rs

COPY src src
RUN cargo install --path .

FROM debian:stable-slim

RUN apt-get update && apt-get install libmariadb-dev -y
COPY --from=builder /usr/local/cargo/bin/darakbang-engine /usr/local/bin/darakbang-engine

ENV ROCKET_PORT 80
EXPOSE 80

CMD ["darakbang-engine"]
