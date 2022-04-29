FROM rustlang/rust:nightly AS build
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian
WORKDIR /var/canteen-server
COPY --from=build /usr/src/app/target/release/server /usr/local/bin/canteen-server
CMD ["sh", "-c", "canteen-server"]
