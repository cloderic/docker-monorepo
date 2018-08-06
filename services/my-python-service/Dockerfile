ARG TAG=latest
FROM monorepo/nice-python:${TAG}

LABEL maintainer="cloderic.mars@gmail.com"

WORKDIR ${ROOT_DIR}/services/my-python-service

ADD Pipfile .
ADD Pipfile.lock .
RUN pipenv install --system

ADD . .
CMD python ./src/main.py
