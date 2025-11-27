FROM debian:bookworm

RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    sudo \
    cmake

RUN git clone https://github.com/cc65/cc65.git && \
    cd cc65 && \
    make && \
    sudo make avail

RUN git clone https://github.com/mist64/msbasic.git && \
    cd msbasic && \
    ./make.sh
