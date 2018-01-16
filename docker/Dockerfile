FROM japaric/armv7-unknown-linux-gnueabihf:latest

#RUN dpkg --add-architecture armhf && \
#    apt-get update && \
#    apt-get install libfoo:armhf

RUN apt-get update \
  && apt-get install -y wget \
  && wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add - \
  && apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 1E9377A2BA9EF27F \
  && echo "deb http://apt.llvm.org/precise/ llvm-toolchain-precise-3.9 main\ndeb http://ppa.launchpad.net/ubuntu-toolchain-r/test/ubuntu precise main" > /etc/apt/sources.list.d/llvm.list \
  && apt-get update \
  && apt-get install -y --no-install-recommends llvm-3.9-dev libclang-3.9-dev clang-3.9 \
  && apt-get purge wget -y --auto-remove \
  && rm -rf /var/lib/apt/lists/*

ENV firmware="1.20171029"

RUN apt-get update \
  && apt-get install -y wget \
  && wget -O - https://github.com/raspberrypi/firmware/archive/$firmware.tar.gz | tar -xzf - -C / --strip-components 1 firmware-$firmware/opt/vc \
  && apt-get purge wget -y --auto-remove \
  && rm -rf /var/lib/apt/lists/*
