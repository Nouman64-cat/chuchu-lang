# Native Linux x64 execution in a container; use --platform linux/amd64 on ARM hosts.
FROM ubuntu:24.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl gnupg build-essential pkg-config libzstd-dev zlib1g-dev libxml2-dev python3 xz-utils \
    && curl -fsSL https://apt.llvm.org/llvm-snapshot.gpg.key | gpg --dearmor -o /usr/share/keyrings/llvm.gpg \
    && echo 'deb [signed-by=/usr/share/keyrings/llvm.gpg] https://apt.llvm.org/noble/ llvm-toolchain-noble-23 main' > /etc/apt/sources.list.d/llvm.list \
    && apt-get update && apt-get install -y --no-install-recommends llvm-23-dev clang-23 \
    && llvm-config-23 --version | grep -E '^23\.1\.' \
    && rm -rf /var/lib/apt/lists/*
RUN curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs -o /tmp/rustup.sh \
    && sh /tmp/rustup.sh -y --default-toolchain 1.99.1 --profile minimal --component rustfmt,clippy \
    && rm /tmp/rustup.sh
ENV PATH="/root/.cargo/bin:/usr/lib/llvm-23/bin:${PATH}"
ENV LLVM_SYS_231_PREFIX=/usr
WORKDIR /workspace
