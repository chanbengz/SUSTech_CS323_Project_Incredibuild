# Use a base image Debian Stable 12
FROM debian:stable-slim

# Install dependencies
RUN apt update && \
    apt install lsb-release wget software-properties-common gnupg build-essential -y

# Install LLVM
RUN wget https://apt.llvm.org/llvm.sh && chmod +x llvm.sh && ./llvm.sh 17


RUN apt install curl -y
# Install Rust using rustup (the official installer for Rust)
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y

# Set environment variables
ENV RUSTUP_HOME=/root/.cargo
ENV CARGO_HOME=/root/.cargo/bin
ENV PATH="${CARGO_HOME}:${PATH}"

# Install rust default toolchain
RUN rustup default stable

# Set environment variables
ENV LLVM_HOME=/usr/lib/llvm-17
ENV PATH="${LLVM_HOME}/bin:${PATH}"
ENV LD_LIBRARY_PATH="${LLVM_HOME}/lib:${LD_LIBRARY_PATH}"
ENV LIBRARY_PATH="${LLVM_HOME}/lib:${LIBRARY_PATH}"
ENV C_INCLUDE_PATH="${LLVM_HOME}/include:${C_INCLUDE_PATH}"
ENV CPLUS_INCLUDE_PATH="${LLVM_HOME}/include:${CPLUS_INCLUDE_PATH}"

# Fix for the error: "error: could not find native static library `Polly`, perhaps an -L flag is missing?" from https://gitlab.com/taricorp/llvm-sys.rs/-/issues/13
RUN apt install libpolly-17-dev -y
# Fix for not finding llvm-config
RUN ln -s /usr/bin/llvm-config-17 /usr/bin/llvm-config

