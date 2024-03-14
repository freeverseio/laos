# This is the build stage for laos. Here we create the binary in a temporary image.
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /laos
COPY . /laos

RUN rustup target add wasm32-unknown-unknown --toolchain nightly 
RUN cargo build --locked --release -p laos

# This is the 2nd stage: a very small image where we copy the laos binary."
FROM docker.io/library/ubuntu:22.04

# Create user
RUN useradd -m -u 1000 -U -s /bin/sh -d /laos laos 

# Copy binary from builder
COPY --from=builder /laos/target/release/laos /usr/local/bin

# Set up directories and permissions
RUN mkdir -p /data /laos/.local/share && \
    chown -R laos:laos /data /laos/.local/share && \
    ln -s /data /laos/.local/share/laos 

# Check if executable works in this container
RUN su laos -c '/usr/local/bin/laos --version'

# Switch to user laos
USER laos

# Expose necessary ports
EXPOSE 9930 9333 9944 30333 30334

# Define volumes
VOLUME ["/data"]

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/laos"]
