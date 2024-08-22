ARG RUST_VERSION=1.80.1

FROM rust:${RUST_VERSION}-slim-bookworm AS builder
WORKDIR /app
COPY backend/. .
RUN \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/mailpass /

FROM debian:bookworm-slim AS final
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser
COPY --from=builder /mailpass /usr/local/bin
RUN chown appuser /usr/local/bin/mailpass
COPY --from=builder /app/config /opt/mailpass/config
RUN chown -R appuser /opt/mailpass
USER appuser
ENV RUST_LOG="mailpass=debug,info"
WORKDIR /opt/mailpass
ENTRYPOINT ["mailpass"]
EXPOSE 3000/tcp