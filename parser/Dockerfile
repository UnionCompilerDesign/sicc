FROM ubuntu:latest

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/sicc

VOLUME ["/usr/sicc/parser", "/usr/sicc/lexer", "/user/sicc/common"]

CMD if [ "$(ls -A /usr/sicc/parser)" ]; then \
        /bin/bash; \
    else \
        echo "Error: /usr/sicc/parser does not exist or is empty" && exit 1; \
    fi

# To build this Dockerfile, use: `docker build -t parser-env .`
# To run this, use: `docker run -it --rm -v $(pwd):/usr/sicc/parser -v $(pwd)/../common:/usr/sicc/common -v $(pwd)/../lexer:/usr/sicc/lexer parser-env`