FROM docker.io/library/ubuntu:22.04

# show backtraces
ENV RUST_BACKTRACE 1

# Create entrypoint script
RUN echo '#!/bin/bash\nset -xeu\n/usr/bin/laos-relay $@' > /usr/bin/entrypoint.sh && \
    chmod +x /usr/bin/entrypoint.sh

# Create user
RUN useradd -m -u 1000 -U -s /bin/sh -d /laos laos 

# Switch to user laos
USER laos

# copy the compiled binary to the container
COPY --chown=laos:laos --chmod=774 target/release/laos-relay /usr/bin/

# check if executable works in this container
RUN /usr/bin/laos-relay --version

# Set the entrypoint script as the entrypoint for the container
ENTRYPOINT ["/usr/bin/entrypoint.sh"]
