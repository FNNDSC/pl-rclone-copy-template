#!/bin/bash -ex

subfolder="$(date +%s)"

docker run --rm --net=host -u $(id -u)$(id -g) \
  -v "$PWD/expected:/incoming:ro" \
  localhost/fnndsc/pl-rclone:testing \
  chrclone --path "/neuro/$subfolder" /incoming /tmp

docker-compose exec openssh-server diff -rq "/neuro/$subfolder" /neuro/example_data
