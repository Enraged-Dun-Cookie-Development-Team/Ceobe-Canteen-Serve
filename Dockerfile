FROM rustlang/rust:nightly AS build
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --bins --features migrator

FROM debian
WORKDIR /var/canteen-server
COPY --from=build /usr/src/app/target/release/server /usr/local/bin/canteen-server
COPY --from=build /usr/src/app/target/release/migrator /usr/local/bin/canteen-migrator
CMD ["sh", "-c", "canteen-migrator ${DB_URL} && canteen-server"]
