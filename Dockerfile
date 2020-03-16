FROM rust as builder

RUN rustup default nightly

WORKDIR /usr/src/darakbang-engine
COPY . .
RUN cargo install --path .

FROM debian:stable-slim

COPY --from=builder /usr/local/cargo/bin/darakbang-engine /usr/local/bin/darakbang-engine

ENV ROCKET_PORT 80
EXPOSE 80

CMD ["darakbang-engine"]
