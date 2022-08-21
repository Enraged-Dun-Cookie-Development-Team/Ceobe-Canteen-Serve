# syntax=docker/dockerfile:1.2
FROM rustlang/rust:nightly AS build
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian
RUN rm -f /etc/apt/apt.conf.d/docker-clean; echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
  apt update && apt-get --no-install-recommends install -y ca-certificates
WORKDIR /var/canteen-server
COPY --from=build /usr/src/app/target/release/server /usr/local/bin/canteen-server
CMD ["sh", "-c", "canteen-server"]
