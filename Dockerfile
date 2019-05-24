FROM rust:1.35 AS builder

# Copy dependency information only.
WORKDIR /home/
COPY ./Cargo.toml ./Cargo.lock ./

# Fetch dependencies to create docker cache layer.
# Workaround with empty main to pass the build, which must be purged after.
RUN  mkdir -p ./src \
  && echo 'fn main() { println!("Dummy") }' > ./src/main.rs \
  && cargo build --release --bin konabb \
  && rm -r ./target/release/.fingerprint/konabb-*

# Cache layer with only my code
COPY ./ ./

# The real build.
RUN cargo build --frozen --release --bin konabb

# The output image
FROM debian:stable-slim

WORKDIR /usr/src/konabb

COPY --from=builder /home/target/release/konabb /usr/src/konabb

ENTRYPOINT [ "/usr/src/konabb" ]

