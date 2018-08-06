ARG TAG=latest
FROM monorepo/nice-python:${TAG}

LABEL maintainer="cloderic.mars@gmail.com"

WORKDIR ${ROOT_DIR}/services/my-python-service

COPY Pipfile .
COPY Pipfile.lock .
RUN pipenv install --system

COPY . .
CMD python ./src/main.py
