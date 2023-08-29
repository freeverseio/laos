FROM docker.io/library/ubuntu:22.04

# show backtraces
ENV RUST_BACKTRACE 1

# Create user
RUN useradd -m -u 1000 -U -s /bin/sh -d /laos laos 

# Switch to user laos
USER laos

# copy the compiled binary to the container
COPY --chown=laos:laos --chmod=774 target/release/laos-relay /usr/bin/

# check if executable works in this container
RUN /usr/bin/laos-relay --version

# ws_port
ENTRYPOINT ["/usr/bin/laos-relay"]