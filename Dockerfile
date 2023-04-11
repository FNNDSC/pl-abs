# TODO https://libreddit.spike.codes/r/rust/comments/126xeyx/exploring_the_problem_of_faster_cargo_docker/

FROM rust:1.68-slim-buster as builder
ARG CARGO_TERM_COLOR=always
WORKDIR /usr/local/src/abs
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/local/src/abs/target/release/abs /usr/local/bin/abs
COPY docker-entrypoint.sh chris_plugin_info.json /
CMD ["abs"]
