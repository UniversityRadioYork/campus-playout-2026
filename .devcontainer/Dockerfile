FROM mcr.microsoft.com/devcontainers/rust:1

# Install extra packages
RUN apt-get update \
  && export DEBIAN_FRONTEND=noninteractive \
  && apt-get -y install --no-install-recommends \
     mold \
     liquidsoap \
     ffmpeg \
  && rm -rf /var/lib/apt/lists/*
