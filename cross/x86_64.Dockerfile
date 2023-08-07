FROM ghcr.io/cross-rs/x86_64-unknown-linux-gnu

RUN apt-get update; apt-get install --yes libclang-dev clang llvm-dev libbluetooth-dev; rm -rf /var/lib/apt/lists/*

