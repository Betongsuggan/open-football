# Base image for building the Rust app
FROM rust:latest AS builder

WORKDIR /usr/src/score-keeper

# Copy the Cargo.toml and source code
COPY Cargo.toml Cargo.lock ./
COPY ./services/score-keeper ./services/score-keeper
COPY ./libs libs/
COPY ./target/release ./target/release

RUN cargo fetch


# Build the Rust app in release mode
RUN cargo build --release --package score-keeper

# Final stage - use a smaller image
FROM rust:slim

COPY --from=builder /usr/src/score-keeper/target/release/score-keeper /usr/local/bin/score-keeper

CMD ["score-keeper"]
