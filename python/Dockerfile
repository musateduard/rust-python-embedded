FROM ubuntu:latest

# install dependencies
ARG DEBIAN_FRONTEND=noninteractive
RUN echo "deb-src http://archive.ubuntu.com/ubuntu/ jammy main restricted" >> /etc/apt/sources.list
RUN apt update
RUN apt upgrade -y
RUN apt install -y git nano build-essential gdb lcov pkg-config libbz2-dev libffi-dev libgdbm-dev libgdbm-compat-dev liblzma-dev libncurses5-dev libreadline6-dev libsqlite3-dev libssl-dev lzma lzma-dev tk-dev uuid-dev zlib1g-dev
RUN apt build-dep -y python3

# clone python repository
ARG PYTHON_VERSION=3.11.3
RUN git clone https://github.com/python/cpython.git

# define working directory
WORKDIR /cpython

# use commit f3909b8bc83675b7ab093dbc558e677558d8e718 for python 3.11.3
RUN git checkout -f f3909b8bc83675b7ab093dbc558e677558d8e718
RUN ./configure --prefix="/rust-python-embedded/python/build" --enable-shared --without-static-libpython --disable-test-modules LDFLAGS="-Wl,-rpath,\\$\$ORIGIN/../lib"
RUN make clean

# create container
# docker create -it --rm --init --add-host host.docker.internal:host-gateway --mount type=bind,source=/mnt/rust/rust-python-embedded/,target=/rust-python-embedded/ --name python_build python_build:latest
# docker start -ia python_build

# build and install to prefix folder
# RUN make -s -j
# RUN make install

CMD ["/bin/bash"]
