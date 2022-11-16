FROM rust:latest

RUN mkdir /var/korat_os
WORKDIR /var/korat_os/

RUN apt-get update -y && apt-get upgrade -y && 
	rustup target add thumbv7em-none-eabihf
