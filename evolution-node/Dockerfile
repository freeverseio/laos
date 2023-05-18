# This is the build stage for laos. Here we create the binary in a temporary image.
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /laos
COPY . /laos

RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the laos binary."
FROM docker.io/library/ubuntu:22.04

COPY --from=builder /laos/target/release/node-template /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /laos laos && \
	mkdir -p /data /laos/.local/share && \
	chown -R laos:laos /data && \
	ln -s /data /laos/.local/share/laos && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/node-template --version

USER laos

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/node-template"]