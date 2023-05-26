FROM docker.io/library/ubuntu:22.04

# Create user
RUN useradd -m -u 1000 -U -s /bin/sh -d /laos laos 

# Copy binary from builder
COPY parachain-template-node /usr/local/bin

# Set up directories and permissions
RUN mkdir -p /data /laos/.local/share && \
    chown -R laos:laos /data /laos/.local/share && \
    ln -s /data /laos/.local/share/laos 

# Check if executable works in this container
RUN su laos -c '/usr/local/bin/parachain-template-node --version'

# Switch to user laos
USER laos

# check if executable works in this container
RUN /usr/local/bin/parachain-template-node --version

# Expose necessary ports
EXPOSE 9930 9333 9944 30333 30334

# Define volumes
VOLUME ["/data"]

# ws_port
CMD ["/usr/local/bin/parachain-template-node"]