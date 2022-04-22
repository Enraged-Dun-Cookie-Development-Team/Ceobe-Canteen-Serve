FROM rustlang/rust:nightly AS build
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian
WORKDIR /var/canteen-server
COPY --from=build target/release/server /usr/local/bin/canteen-server
CMD ["canteen-server"]
