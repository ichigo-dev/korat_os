FROM rust:latest

RUN mkdir /var/korat_os
WORKDIR /var/korat_os/

RUN apt-get update -y && apt-get upgrade -y && \
	apt-get install -y qemu-system qemu-system-common qemu-utils && \
	rustup override set nightly && \
	rustup component add rust-src && \
	rustup component add llvm-tools-preview && \
	cargo install bootimage
