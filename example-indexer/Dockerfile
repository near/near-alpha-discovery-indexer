FROM rust:1.67.1 as build

# create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Deps
RUN apt update
RUN apt install -y libprotobuf-dev protobuf-compiler
RUN apt install -y make clang pkg-config libssl-dev

RUN apt install libpq-dev
RUN apt install -y cmake

# this build step will cache your dependencies
#RUN cargo update
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
#RUN rm ./target/release/deps/nearsocial-indexer*
RUN cargo build --release

# our final base
FROM ubuntu:20.04

# copy the build artifact from the build stage
COPY --from=build /app/target/release/nearsocial-indexer .

# set the startup command to run your binary
ENTRYPOINT ["./nearsocial-indexer"]