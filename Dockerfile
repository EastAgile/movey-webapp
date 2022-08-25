FROM rust:latest
RUN apt-get update \
    && apt-get install -y postgresql \
    && rm -rf /var/lib/apt/lists/* \
    && cargo install diesel_cli --no-default-features --features postgres \
    && cargo install cargo-watch
WORKDIR /app
COPY . /app

EXPOSE 17001
ENTRYPOINT /bin/sh -c "until diesel setup;                  \
     do echo 'Migrations failed, retrying in 5 seconds...'; \
     sleep 5; done; cargo watch -x run"
