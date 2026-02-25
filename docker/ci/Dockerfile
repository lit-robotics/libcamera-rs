# Minimal CI runner image that matches the GitHub Actions workflow.
# Build per libcamera version:
#   docker build -f docker/ci/Dockerfile --build-arg LIBCAMERA_VERSION=v0.5.1 -t libcamera-ci:v0.5.1 .
#
# Then run against your workspace:
#   docker run --rm -v "$PWD:/workspace" -w /workspace libcamera-ci:v0.5.1 ./docker/ci/entrypoint.sh

ARG UBUNTU_VERSION=22.04
FROM ubuntu:${UBUNTU_VERSION}

ARG LIBCAMERA_VERSION=v0.7.0
ENV DEBIAN_FRONTEND=noninteractive

# Install build prerequisites for libcamera and Rust.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential git ninja-build pkg-config clang \
        python3 python3-pip python3-jinja2 python3-yaml python3-ply \
        libyaml-dev libssl-dev curl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Meson from apt on 22.04 is too old for older libcamera tags; upgrade via pip.
RUN pip3 install --no-cache-dir --upgrade pip meson

# Install libcamera (matching the workflow: only vimc pipeline for speed).
RUN git clone --depth 1 --branch ${LIBCAMERA_VERSION} https://git.libcamera.org/libcamera/libcamera.git /tmp/libcamera && \
    cd /tmp/libcamera && \
    meson build -Dipas=vimc -Dpipelines=vimc && \
    ninja -C build install && \
    rm -rf /tmp/libcamera

# Install Rust toolchain (stable) with rustfmt/clippy.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --profile minimal --component rustfmt,clippy && \
    . "$HOME/.cargo/env" && \
    rustc --version && cargo --version

ENV PATH="/root/.cargo/bin:${PATH}"
# Ensure pkg-config can find the installed libcamera.
ENV PKG_CONFIG_PATH="/usr/local/lib/pkgconfig:/usr/local/lib/x86_64-linux-gnu/pkgconfig:${PKG_CONFIG_PATH}"

WORKDIR /workspace
COPY docker/ci/entrypoint.sh /usr/local/bin/entrypoint.sh
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
