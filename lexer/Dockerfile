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

VOLUME ["/usr/sicc/lexer", "/user/sicc/common"]

CMD if [ "$(ls -A /usr/sicc/lexer)" ]; then \
        /bin/bash; \
    else \
        echo "Error: /usr/sicc/lexer does not exist or is empty" && exit 1; \
    fi

# To build this Dockerfile, use: `docker build -t lexer-env .`
# To run this, use: `docker run -it --rm -v $(pwd):/usr/sicc/lexer -v $(pwd)/../common:/usr/sicc/common lexer-env`