ARG TAG=latest
FROM monorepo/fancy-rust:${TAG}

LABEL maintainer="cloderic.mars@gmail.com"

WORKDIR ${ROOT_DIR}/services/my-rust-service

ADD . .
RUN cargo build --release

CMD target/release/my-rust-service
