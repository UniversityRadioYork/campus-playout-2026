FROM rust:1-trixie

# Install extra packages
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends liquidsoap

# Install SQLx
RUN cargo install sqlx-cli --no-default-features --features sqlite
