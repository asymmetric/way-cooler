FROM rust:1.22.1
RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    cmake \
    git \
    libcairo2-dev \
    libdbus-1-dev \
    libegl1-mesa-dev \
    libgbm-dev \
    libgles2-mesa-dev \
    libinput-dev \
    liblua5.3-dev \
    libpixman-1-dev \
    libsystemd-dev \
    libudev-dev \
    libwayland-dev \
    libwayland-egl1-mesa \
    libx11-dev \
    libx11-xcb-dev \
    libxcb-composite0-dev \
    libxcb-ewmh-dev \
    libxcb-image0-dev \
    libxcb-xfixes0 \
    libxcb-xkb-dev \
    libxcb1-dev \
    libxkbcommon-dev \
    pkg-config \
    wayland-protocols \
 && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/wlc
RUN git clone https://github.com/Cloudef/wlc . && git submodule update --init --recursive && \
  cmake -DCMAKE_BUILD_TYPE=Upstream -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_INSTALL_LIBDIR=lib . && make && make install
RUN cargo install --git https://github.com/way-cooler/way-cooler way-cooler
