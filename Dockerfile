FROM ubuntu:latest

ENV DEBIAN_FRONTEND=noninteractive

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

ENV LLVM_SYS_170_PREFIX=/usr/lib/llvm-17
ENV PATH="/usr/lib/llvm-17/bin:$PATH"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/sicc

COPY . . 

VOLUME ["/usr/sicc"]

CMD if [ "$(ls -A /usr/sicc)" ]; then \
        /bin/bash; \
    else \
        echo "Error: /usr/sicc does not exist or is empty" && exit 1; \
    fi

# To build this, use: `docker build -t sicc-env .`
# To run this, use: `docker run -it --rm -v $(pwd):/usr/sicc sicc-env`