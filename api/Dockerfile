FROM rust:alpine AS deps
RUN cargo init --name dependencies
COPY Cargo.toml .
COPY --from=workspace Cargo.lock .
RUN cargo build --release

FROM rust:alpine AS builder
RUN apk add libpq-dev
COPY --from=workspace Cargo.lock .
COPY --from=deps target/ target
COPY --from=deps $CARGO_HOME $CARGO_HOME
COPY . .
RUN touch src/main.rs
RUN cargo build --release

FROM rust:alpine
COPY --from=builder target/release/vocab_map_api /usr/local/bin/
COPY --from=workspace .env .
ENTRYPOINT ["vocab_map_api", "api"]
