#!/bin/bash -ex

outgoing="$(mktemp -d)"

docker run --rm --userns=host --net=host -u "$(id -u):$(id -g)" \
  -v "$outgoing:/outgoing:rw" \
  localhost/fnndsc/pl-rclone:testing \
  chrclone --path "/neuro/example_data" /outgoing

diff -rq expected "$outgoing"
rm -rv "$outgoing"
