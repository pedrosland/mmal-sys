FROM rustembedded/cross:armv7-unknown-linux-gnueabihf-0.1.16

#RUN dpkg --add-architecture armhf && \
#    apt-get update && \
#    apt-get install libfoo:armhf

ENV firmware="1.20190925"

RUN apt-get update \
  && apt-get install -y wget \
  && wget -O - https://github.com/raspberrypi/firmware/archive/$firmware.tar.gz | tar -xzf - -C / --strip-components 2 firmware-$firmware/hardfp/opt/vc \
  && apt-get purge wget -y --auto-remove \
  && rm -rf /var/lib/apt/lists/*

RUN apt-get update \
  && apt-get install -y --no-install-recommends llvm-3.9-dev libclang-3.9-dev clang-3.9 \
  && rm -rf /var/lib/apt/lists/*

