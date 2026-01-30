FROM rust:1.91-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.inator.rs && cargo build --release && rm -rf src target/release/deps/namer_inator* target/release/namer-inator

COPY src/ src/
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/namer-inator .
COPY names-list.inator .

EXPOSE 8080

CMD ["./namer-inator"]
