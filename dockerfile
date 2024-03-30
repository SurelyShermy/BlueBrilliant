# First stage: build the application
FROM rust:latest as builder

# Create the working directory
WORKDIR /usr/src/blue_brilliant

# Copy your Rust project's manifests
COPY ./BlueBrilliant/Cargo.toml ./Cargo.toml
COPY ./BlueBrilliant/Cargo.lock ./Cargo.lock
COPY ./BlueBrilliant/Rocket.toml ./Rocket.toml

# Cache the dependencies by creating a dummy project
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/*.rs

# Now copy in the actual source code of your application
COPY ./BlueBrilliant/src ./src
RUN ls
# Build the project for release - this builds your actual application
RUN cargo build --release

# Second stage: prepare the runtime environment
# Use rust:latest as the base image for the runtime environment to match the build environment
FROM rust:latest as runtime

WORKDIR /usr/src/blue_brilliant


# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/blue_brilliant/target/release/BlueBrilliant /usr/src/blue_brilliant

# Set the binary as the container's entry point
CMD ["./BlueBrilliant"]