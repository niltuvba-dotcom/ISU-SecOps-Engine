# Stage 1: Build Environment
FROM rust:1.77-alpine3.18 as builder

# Add musl dependencies
RUN apk add --no-cache musl-dev bind-tools linux-headers build-base

WORKDIR /usr/src/aetheris
COPY . .

# Build for release
RUN cargo build --release

# Stage 2: Runtime Environment
FROM alpine:3.18

RUN apk add --no-cache libgcc logrotate
COPY --from=builder /usr/src/aetheris/target/release/aetheris-engine /usr/local/bin/aetheris

# Default Web UI port
EXPOSE 8080

ENTRYPOINT ["aetheris"]
CMD ["web", "--port", "8080"]
