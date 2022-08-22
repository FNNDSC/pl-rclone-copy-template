# Rclone Copy ChRIS Plugin _Template_

This repository provides source code which can be built
into a _ChRIS_ plugin which uses `rclone copy`
to either push or retrieve files from a remote.

NB: this repository itself is not a _ChRIS_ plugin.
The _ChRIS_ admin must provide their own `rclone.conf`,
build their own image, and customize `chris_plugin_info.json`
to suit their own needs.

## Usage

`chrclone` is an adapter for running `rclone copy` from
_ChRIS_ plugin usage spec. `chrclone` can be called as
either an _fs_ or _ds_ plugin.

```shell
# fs-type plugin usage: fetch from remote into ChRIS feed.
# translated as: rclone copy remote_name:/neuro/my_data /outgoing
chrclone --path /neuro/my_data /outgoing

# ds-type plugin usage: push from ChRIS feed into remote.
# translated as: rclone copy /incoming remote_name:/neuro/my_data
chrclone --path /neuro/my_data /incoming /tmp
```

These are just examples. It's not recommended to use `chrclone` outside of _ChRIS_.

## Building

Pass `rclone.conf` as a string of base64-encoded data into the image during build.
`rclone.conf` must specify at least one remote.

```shell
DOCKER_BUILDKIT=1 docker build -t fnndsc/pl-rclone \
  --build-arg RCLONE_CONFIG_BASE64="$(base64 < testing/ssh/rclone.conf)" .
```

## Troubleshooting ERROR : Failed to save config after 10 tries

https://github.com/rclone/rclone/issues/3655

Rclone sometimes wants to amend the configuration file.
It is recommended to run `rclone --config ./my_config.conf copy ...`
once on-the-metal first before building this _ChRIS_ plugin.

## Limitations on Filtering

It is not possible to repeat the `--include`, `exclude`, nor `--filter` flags.
This is a limitation of the _ChRIS_ specification itself.
