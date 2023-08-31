# This is the build stage for laos. Here we create the binary in a temporary image.
FROM rust:slim-buster as builder

WORKDIR /laos
COPY . /laos

RUN apt update
# RUN apt upgrade -yy
RUN apt-get install -y clang libssl-dev llvm libudev-dev protobuf-compiler pkg-config

RUN rustup toolchain install nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly 
RUN cargo build --release -p laos-relay

# This is the 2nd stage: a very small image where we copy the laos binary."
FROM docker.io/library/ubuntu:22.04

# Create user
RUN useradd -m -u 1000 -U -s /bin/sh -d /laos laos 

# Copy binary from builder
COPY --from=builder /laos/target/release/laos-relay /usr/bin

# Check if executable works in this container
RUN su laos -c '/usr/bin/laos-relay --version'

# Create entrypoint script
RUN echo '#!/bin/bash\nset -xeu\n/usr/bin/laos-relay $@' > /usr/bin/entrypoint.sh && \
    chmod +x /usr/bin/entrypoint.sh

# Switch to user laos
USER laos

# Set the entrypoint script as the entrypoint for the container
ENTRYPOINT ["/usr/bin/entrypoint.sh"]
