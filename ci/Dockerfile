FROM rustlang/rust:nightly
RUN cargo install diesel_cli

FROM node:latest
RUN apt-get update -qy && apt-get install -y ruby-dev && gem install dpl