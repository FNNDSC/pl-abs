FROM rust:1.68-slim-buster as base
FROM base as builder
ARG CARGO_TERM_COLOR=always

WORKDIR /usr/local/src/abs

COPY . .

# Build using persistent cache, requires Docker buildx
# More info: https://github.com/rust-lang/cargo/issues/2644
RUN --mount=type=cache,target=/usr/local/cargo,from=base,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/local/src/abs/target/release/abs /usr/local/bin/abs
COPY docker-entrypoint.sh chris_plugin_info.json /
CMD ["abs"]
