FROM rust:1.68-slim-buster as base
FROM base as builder
ARG CARGO_TERM_COLOR=always

WORKDIR /usr/local/src/abs


# Build dependencies only to take advantage of layer caching
# See https://github.com/rust-lang/cargo/issues/2644
#
# Note: we cannot use buildx cache mounts. https://github.com/docker/build-push-action/issues/716
#RUN --mount=type=cache,target=/usr/local/cargo,from=base,source=/usr/local/cargo \
#    --mount=type=cache,target=target \
#    cargo build --release

# Copy files which specify dependencies
COPY Cargo.toml Cargo.lock ./
# Create a dummy file, to allow compilation to succeed
RUN mkdir src && touch src/lib.rs
# Build dependencies only
RUN cargo build --release

# Copy sources and build actual probject
COPY src src
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/local/src/abs/target/release/abs /usr/local/bin/abs
COPY docker-entrypoint.sh chris_plugin_info.json /
CMD ["abs"]
