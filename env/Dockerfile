FROM --platform=linux/amd64 rust:1.57.0

# コンテナ実行時に -v オプションでマウントされるのを前提としている。
RUN apt-get update & apt-get install -y pkg-config automake libtool git
RUN wget https://github.com/Pro/raspi-toolchain/releases/latest/download/raspi-toolchain.tar.gz
RUN tar xfz raspi-toolchain.tar.gz --strip-components=1 -C /opt
ENV PATH=/opt/cross-pi-gcc/bin:/opt/cross-pi-gcc/libexec/gcc/arm-linux-gnueabihf/8.3.0:$PATH

RUN git clone https://github.com/alsa-project/alsa-lib.git
WORKDIR /alsa-lib
RUN git checkout v1.2.4
RUN libtoolize --force --copy --automake
RUN aclocal
RUN autoheader
RUN automake --foreign --copy --add-missing
RUN autoconf
RUN CC=arm-linux-gnueabihf-gcc ./configure --host=arm-linux
RUN sed -i -e "31i #include <linux/limits.h>" src/ucm/ucm_subs.c
RUN sed -i -e "39i #include <linux/limits.h>" src/ucm/main.c
RUN sed -i -e "36i #include <linux/limits.h>" src/dlmisc.c
RUN sed -i -e "13i #include <linux/limits.h>" src/topology/tplg_local.h
RUN make
RUN make DESTDIR=/opt/cross-pi-gcc/arm-linux-gnueabihf install
ENV PKG_CONFIG_SYSROOT_DIR=/opt/cross-pi-gcc/arm-linux-gnueabihf
ENV PKG_CONFIG_PATH=/opt/cross-pi-gcc/arm-linux-gnueabihf/usr/lib/pkgconfig

USER 1000:1000

WORKDIR /Enviro-By-Rust
RUN rustup target add arm-unknown-linux-gnueabihf

CMD ["cargo", "build", "--target", "arm-unknown-linux-gnueabihf", "--release"]