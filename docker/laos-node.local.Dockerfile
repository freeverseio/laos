FROM docker.io/library/ubuntu:24.04

# show backtraces
ENV RUST_BACKTRACE 1

# Create user

# Set up directories and permissions
RUN mkdir -p /data /laos/.local/share && \
    ln -s /data /laos/.local/share/laos 


# copy the compiled binary to the container
COPY --chmod=777 target/release/laos /usr/bin/laos

# check if executable works in this container
RUN /usr/bin/laos --version

# Expose necessary ports
EXPOSE 9930 9333 9944 30333 30334

# Define volumes
VOLUME ["/data"]

# ws_port
ENTRYPOINT ["/usr/bin/laos"]
