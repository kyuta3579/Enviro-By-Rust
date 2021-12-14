FROM --platform=linux/amd64 rust:1.57.0

# コンテナ実行時に -v オプションでマウントされるのを前提としている。
RUN wget https://github.com/Pro/raspi-toolchain/releases/latest/download/raspi-toolchain.tar.gz
RUN tar xfz raspi-toolchain.tar.gz --strip-components=1 -C /opt
ENV PATH=/opt/cross-pi-gcc/bin:/opt/cross-pi-gcc/libexec/gcc/arm-linux-gnueabihf/8.3.0:$PATH

USER 1000:1000

WORKDIR /Enviro-By-Rust
RUN rustup target add arm-unknown-linux-gnueabihf

CMD ["cargo", "build", "--target", "arm-unknown-linux-gnueabihf", "--release"]