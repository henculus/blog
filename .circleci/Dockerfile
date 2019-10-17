FROM rustlang/rust:nightly
RUN cargo install diesel_cli && \
     apt-get update -qy && \
     apt-get install -y ruby-dev curl && \
     curl -sL https://deb.nodesource.com/setup_10.x | bash - && \
     apt-get install nodejs && \
     gem install dpl