# Rclone Copy ChRIS Plugin _Template_

This repository provides source code which can be built
into a _ChRIS_ *fs*-type plugin which uses `rclone copy`
to retrieve files from a remote. 

NB: this repository itself is not a _ChRIS_ plugin. When it
is built, the rclone config (remote configuration and
authentication) is built into the image.



```shell
DOCKER_BUILDKIT=1 docker build -t localhost/fnndsc/pl-rclone --build-arg RCLONE_CONFIG_BASE64="$(base64 < testing/ssh/rclone.conf)" .
```
