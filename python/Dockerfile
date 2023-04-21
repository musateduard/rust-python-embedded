FROM ubuntu:latest

ARG DEBIAN_FRONTEND=noninteractive
RUN echo "deb-src http://archive.ubuntu.com/ubuntu/ jammy main restricted" >> /etc/apt/sources.list
RUN apt update
RUN apt upgrade -y
RUN apt install -y git nano build-essential gdb lcov pkg-config libbz2-dev libffi-dev libgdbm-dev libgdbm-compat-dev liblzma-dev libncurses5-dev libreadline6-dev libsqlite3-dev libssl-dev lzma lzma-dev tk-dev uuid-dev zlib1g-dev
RUN apt build-dep -y python3
RUN git clone https://github.com/python/cpython.git

WORKDIR /cpython

RUN git checkout -f 814da78353ba31d18a450d1a1aa4321cc17b985c
RUN ./configure --prefix="/home/python_portable" --with-pkg-config=yes --enable-shared --disable-test-modules LDFLAGS="-Wl,-rpath,\\$\$ORIGIN/../lib"
RUN make clean

# run these commands to build and install to the selected folder
# RUN make -s -j
# RUN make install

CMD ["/bin/bash"]
