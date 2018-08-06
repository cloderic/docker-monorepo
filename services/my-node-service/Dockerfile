ARG TAG=latest
FROM monorepo/cool-node:${TAG}

LABEL maintainer="cloderic.mars@gmail.com"

WORKDIR ${ROOT_DIR}/services/my-node-service

COPY ./package.json ./
RUN npm install

COPY . .
CMD npm start
