FROM postgres

WORKDIR /root
COPY zprofile .zprofile
COPY config.toml .cargo/config.toml

RUN apt-get update -qqy \
 && apt-get install -qqy build-essential curl git zsh \
 && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
 
RUN git clone https://github.com/jeffs/edu-axum
WORKDIR /root/edu-axum
ENV PATH="/root/.cargo/bin:$PATH"
RUN cargo build
CMD cargo run
