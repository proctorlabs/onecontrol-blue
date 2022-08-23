# Rust alpine/musl builder
FROM rust:1.63-slim-bullseye as app-builder
RUN apt-get update && apt-get install -y libdbus-1-dev build-essential
COPY Cargo.toml Cargo.lock /app/
COPY rvlink-bridge /app/rvlink-bridge
COPY rvlink-common /app/rvlink-common
COPY rvlink-proto /app/rvlink-proto
WORKDIR /app
RUN cargo build --release

# Target container
FROM debian:bullseye-slim as service
RUN apt-get update && apt-get install -y --no-install-recommends dbus && \
    mkdir -p /var/run/dbus && \
    rm -rf /var/lib/apt/lists/*
COPY --from=app-builder /app/target/release/rvlink-bridge /app/rvlink-bridge
WORKDIR /app
ENTRYPOINT [ "./rvlink-bridge" ]
