FROM ghcr.io/cross-rs/armv7-unknown-linux-gnueabihf:edge

#FROM rustembedded/cross:armv7-unknown-linux-gnueabihf-0.2.1
ENV DEBIAN_FRONTEND=noninteractive
RUN dpkg --add-architecture armhf && apt-get update

# install needed dependencies that are needed :
RUN apt-get install --assume-yes pkg-config:armhf 
RUN apt-get install --assume-yes libdbus-1-dev:armhf

