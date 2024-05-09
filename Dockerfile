FROM docker.io/library/rust:1.78-alpine as builder

# Compile dependencies (musl, openssl)
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconf git

WORKDIR /wd
COPY . /wd
RUN cargo build --bins --release

# Use scratch image to reduce image size
FROM scratch
ARG version=0.1.0
ARG release=unreleased
LABEL name="player-Rust" \
      maintainer="Fachschaft IT, HS-Esslingen" \
      version=${version} \
      release=${release} \
      summary="A client for BitWars written in Rust using Tokio" \
      description="A client for BitWars written in Rust using the Axum webserver with Tokio runtime"
EXPOSE 3000

COPY --from=builder /wd/target/release/player-Rust /
CMD ["./player-Rust"]
