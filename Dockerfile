# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

WORKDIR /usr/src/cert-checker

COPY ./ ./
# ENV OPENSSL_STATIC=yes
# ENV OPENSSL_LIB_DIR=/usr/lib/
# ENV OPENSSL_INCLUDE_DIR=/usr/include/
RUN cargo build --release

RUN cargo install --path .

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM ubuntu:18.04
RUN apt-get update &&\
    apt-get install --no-install-recommends libssl1.1 -y  &&\
    rm -rf /var/lib/apt/lists/*

COPY --from=cargo-build /usr/local/cargo/bin/cert-checker /usr/local/bin/cert-checker
ENTRYPOINT [ "cert-checker" ]
