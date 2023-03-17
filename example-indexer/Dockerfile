FROM rust:1.66 as build

# create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

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
RUN cargo update
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/example_indexer*
RUN cargo build --release

# our final base
FROM rust:1.66

# copy the build artifact from the build stage
COPY --from=build /app/target/release/example-indexer /
COPY --from=build /app/Cargo.toml /

# initializing the app
RUN ./example-indexer init 

# set the startup command to run your binary
CMD ["./example-indexer run"]