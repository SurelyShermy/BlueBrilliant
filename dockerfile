# First stage: build the application
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# Copy your manifests
COPY ./Bluebrilliant/Cargo.toml ./Cargo.toml
COPY ./Bluebrilliant/Cargo.lock ./Cargo.lock

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./Bluebrilliant/src ./src

# Build for release
RUN rm ./target/release/deps/server*
RUN cargo build --release

# Second stage: prepare the runtime environment
FROM debian:buster-slim

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/server /usr/local/bin/server

# Set the binary as the container's entry point
CMD ["server"]