FROM ubuntu:latest

ARG DEBIAN_FRONTEND=noninteractive
RUN echo "deb-src http://archive.ubuntu.com/ubuntu/ jammy main restricted" >> /etc/apt/sources.list
RUN apt update
RUN apt upgrade -y
RUN apt install -y git nano build-essential gdb lcov pkg-config libbz2-dev libffi-dev libgdbm-dev libgdbm-compat-dev liblzma-dev libncurses5-dev libreadline6-dev libsqlite3-dev libssl-dev lzma lzma-dev tk-dev uuid-dev zlib1g-dev
RUN apt build-dep -y python3
RUN git clone https://github.com/python/cpython.git

WORKDIR /cpython

# use commit nr. f3909b8bc83675b7ab093dbc558e677558d8e718 for python version 3.11.3
RUN git checkout -f f3909b8bc83675b7ab093dbc558e677558d8e718
RUN ./configure --prefix="/rust-python-embedded/python/build" --with-pkg-config=yes --enable-shared --disable-test-modules LDFLAGS="-Wl,-rpath,\\$\$ORIGIN/../lib"
RUN make clean

# create container
# docker create -it --rm --init --add-host host.docker.internal:host-gateway --mount type=bind,source=/mnt/rust/rust-python-embedded/,target=/rust-python-embedded/ --name python_build python_build:latest
# docker start -ia python_build

# run these commands to build and install to the selected folder
# RUN make -s -j
# RUN make install

CMD ["/bin/bash"]
