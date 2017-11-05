FROM ubuntu:xenial-20170915

RUN apt update && apt install -y \
        build-essential \
        cmake \
        gcc-arm-none-eabi \
        gdb-arm-none-eabi \ 
        libusb-1.0-0-dev \
        wget \
    && \
    apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN mkdir nucleo && cd nucleo && \
    wget https://github.com/texane/stlink/archive/1.3.0.tar.gz && \
    tar xvf 1.3.0.tar.gz && cd stlink-1.3.0 && \
    make release && cd build/Release && make install

ENV LD_LIBRARY_PATH="/usr/local/lib:${LD_LIBRARY_PATH}"

RUN apt update && apt install -y \
        curl \
        libssl-dev \
        pkg-config \
    && \
    apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN wget https://sh.rustup.rs -O /tmp/rustup-init && \
    bash /tmp/rustup-init --default-toolchain nightly -y && \
    rm /tmp/rustup-init

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install xargo

RUN rustup component add rust-src