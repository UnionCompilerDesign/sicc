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

WORKDIR /usr/starter_code

VOLUME ["/usr/starter_code/sts", "/user/starter_code/common"]

CMD if [ "$(ls -A /usr/starter_code/sts)" ]; then \
        /bin/bash; \
    else \
        echo "Error: /usr/starter_code/sts does not exist or is empty" && exit 1; \
    fi

# To build this Dockerfile, use: `docker build -t sts-env .`
# To run this, use: `docker run -it --rm -v $(pwd):/usr/starter_code/sts -v $(pwd)/../common:/usr/starter_code/common sts-env`