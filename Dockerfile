FROM rustlang/rust:nightly AS chef
RUN cargo install cargo-chef
WORKDIR /usr/src/app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner usr/src/app/recipe.json recipe.json
COPY . .
RUN cargo build --release --bins --features migrator

FROM debian
WORKDIR /var/canteen-server
COPY --from=build /usr/src/app/target/release/server /usr/local/bin/canteen-server
COPY --from=build /usr/src/app/target/release/migrator /usr/local/bin/canteen-migrator
CMD ["sh", "-c", "canteen-migrator ${DB_URL} && canteen-server"]
