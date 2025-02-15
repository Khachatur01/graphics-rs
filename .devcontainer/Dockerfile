FROM rust:1.84

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install nodejs -y
RUN apt-get install npm -y

RUN cargo install wasm-pack

EXPOSE 8080

ENTRYPOINT ["bash"]
