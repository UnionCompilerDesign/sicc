# Ubuntu as the base image
FROM ubuntu:latest

# Set environment variable to avoid user interaction during installation
ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    git \
    pkg-config \
    libssl-dev \
    llvm-17 \
    llvm-17-dev \
    llvm-17-tools \
    clang-17 \
    zlib1g-dev \
    libllvm17 \
    libfontconfig1-dev \
    libpolly-17-dev \ 
    libzstd-dev \
    && rm -rf /var/lib/apt/lists/*

# Set LLVM config environment variables
ENV LLVM_SYS_170_PREFIX=/usr/lib/llvm-17
ENV PATH="/usr/lib/llvm-17/bin:$PATH"

# Install Rust 
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory in the container
WORKDIR /usr/compiler_core

# Mount the local directories 
VOLUME ["/usr/compiler_core/ir", "/usr/compiler_core/frontend"]

CMD if [ "$(ls -A /usr/compiler_core/ir)" ]; then \
        /bin/bash; \
    else \
        echo "Error: /usr/compiler_core/ir does not exist or is empty" && exit 1; \
    fi

# To build this, use: `docker build -t ir-env .`
# To run this, use: `docker run -it --rm -v $(pwd):/usr/compiler_core/ir -v $(pwd)/../frontend:/usr/compiler_core/frontend ir-env`