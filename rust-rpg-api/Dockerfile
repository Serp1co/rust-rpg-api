#Rust builder image
FROM rust:1.73.0-slim-buster as build
RUN apt-get update
#install pkg-config for library linking
RUN apt-get -y install pkg-config
#install openssl devel for tsl
RUN apt-get -y install libssl-dev
RUN mkdir -p /app/tls
WORKDIR /app
# copy your source tree
COPY *.pem /app/tsl/
COPY ./src ./src
# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

# final debian image with compiled rust-app
FROM debian:buster-slim
ENV APP_NAME="rust-rpg-user"
RUN apt-get update
RUN apt-get -y install openssl
# copy the build artifact from the build stage
COPY --from=build /app/target/release/${APP_NAME} /app
COPY --from=build /app/tsl/* /
# set the startup command to run your binary
CMD ["/app"]