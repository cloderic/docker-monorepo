FROM node:8

ARG TAG=latest
ARG GIT_SHA1=latest

LABEL maintainer="cloderic.mars@gmail.com"

ENV MONOREPO_SHA1 ${GIT_SHA1}
ENV MONOREPO_VERSION ${TAG}

ENV ROOT_DIR /app

WORKDIR ${ROOT_DIR}/bases/cool-node
ADD . .
