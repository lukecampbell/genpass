FROM rust:1.93.0-slim-bookworm

# Metadata
LABEL org.opencontainers.image.authors="Luke Campbell <luke@axds.co>"
LABEL org.opencontainers.image.url="https://git.axiom/axiom/genpass/"
LABEL org.opencontainers.image.source="https://git.axiom/axiom/genpass/"
LABEL org.opencontainers.image.licenses="MIT"



# Build the release binary
WORKDIR /opt/genpass
COPY src ./src
COPY README.md LICENSE Cargo.toml ./

RUN cargo build --release


# Copy release binary to fresh buster-slim image
FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y ca-certificates libssl-dev openssl \
    && apt-get clean \
    && groupadd --system appuser \
    && useradd --system --create-home --gid appuser appuser
WORKDIR /home/appuser
COPY --from=0 /opt/genpass/target/release/genpass /usr/bin/genpass
USER appuser
ENTRYPOINT ["/usr/bin/genpass"]
