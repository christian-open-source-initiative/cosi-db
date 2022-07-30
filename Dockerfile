# https://blog.mgattozzi.dev/caching-rust-docker-builds/
FROM rust:1.62
RUN mkdir /mnt/cosi/
COPY dummy.rs /mnt/cosi/
COPY Cargo.lock /mnt/cosi/
COPY Cargo.toml /mnt/cosi/
RUN sed -i 's|src/main.rs|dummy.rs|' /mnt/cosi/Cargo.toml
RUN cd /mnt/cosi/ && cargo build --release
RUN sed -i 's|dummy.rs|src/main.rs|' /mnt/cosi/Cargo.toml
COPY . /mnt/cosi
CMD cd /mnt/cosi && cargo run
