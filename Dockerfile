FROM ubuntu:18.04

WORKDIR /home

RUN apt-get update && \
        apt-get install -y gcc make git binutils libc6-dev

