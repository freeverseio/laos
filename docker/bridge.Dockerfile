# This is the build stage for laos. Here we create the binary in a temporary image.
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /laos
COPY . /laos

RUN SKIP_WASM_BUILD=1 cargo build --release

# This is the 2nd stage: a very small image where we copy the laos binary."
FROM docker.io/debian:bullseye-slim

# Copy binary from builder
COPY --from=builder /laos/target/release/laos-relay /usr/bin

# Create user
RUN useradd -m -u 1000 -U -s /bin/sh -d /laos laos 

# Check if executable works in this container
RUN su laos -c '/usr/bin/laos-relay --version'

# Create entrypoint script
RUN echo '#!/bin/bash\nset -xeu\n/usr/bin/laos-relay $@' > /usr/bin/entrypoint.sh && \
    chmod +x /usr/bin/entrypoint.sh

# Switch to user laos
USER laos

# Set the entrypoint script as the entrypoint for the container
ENTRYPOINT ["/usr/bin/entrypoint.sh"]
