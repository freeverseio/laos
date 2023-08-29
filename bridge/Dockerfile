# This is the build stage for laos. Here we create the binary in a temporary image.
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /laos
COPY . /laos

RUN rustup target add wasm32-unknown-unknown --toolchain nightly 
RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the laos binary."
FROM docker.io/library/ubuntu:22.04

# Create user
RUN useradd -m -u 1000 -U -s /bin/sh -d /laos laos 

# Copy binary from builder
COPY --from=builder /laos/target/release/laos-relay /usr/local/bin

# Check if executable works in this container
RUN su laos -c '/usr/local/bin/laos-relay --version'

# Switch to user laos
USER laos

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/laos-relay"]