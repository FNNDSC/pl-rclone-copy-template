FROM docker.io/python:3.10.5-slim-alpine

ARG PLUGIN_NAME
ARG RCLONE_CONFIG
ARG PLUGIN_DESCRIPTION="A ChRIS fs-type plugin wrapper for rclone copy"
ARG PLUGIN_URL="https://github.com/FNNDSC/pl-rclone-copy-template"
ARG PLUGIN_AUTHOR="FNNDSC <dev@babyMRI.org>"

LABEL org.opencontainers.image.authors=$PLUGIN_AUTHOR \
      org.opencontainers.image.url=$PLUGIN_URL \
      org.opencontainers.image.title=$PLUGIN_NAME \
      org.opencontainers.image.description=$PLUGIN_DESCRIPTION

WORKDIR /usr/local/src/chrclone

COPY requirements.txt .
RUN pip install -r requirements.txt

COPY . .
ARG extras_require=none
RUN pip install ".[${extras_require}]"

CMD ["chrclone", "--help"]
