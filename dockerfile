# First stage: build the application
FROM rust:latest as builder

# Create the working directory
WORKDIR /usr/src/BlueBrilliant


COPY ./BlueBrilliant .
# Build the project for release - this builds your actual application
RUN cargo install --path .

# Second stage: prepare the runtime environment
# Use rust:latest as the base image for the runtime environment to match the build environment
FROM rust:latest

RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/BlueBrilliant /usr/local/bin/BlueBrilliant

# Set the binary as the container's entry point
CMD ["BlueBrilliant"]