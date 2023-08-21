# Rclone Copy ChRIS Plugin _Template_

[![MIT License](https://img.shields.io/github/license/fnndsc/pl-rclone-copy-template)](https://github.com/FNNDSC/pl-rclone-copy-template/blob/main/LICENSE)
[![CI](https://github.com/FNNDSC/pl-rclone-copy-template/actions/workflows/test.yml/badge.svg)](https://github.com/FNNDSC/pl-rclone-copy-template/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/FNNDSC/pl-rclone-copy-template/graph/badge.svg?token=9DX0QX1CGL)](https://codecov.io/gh/FNNDSC/pl-rclone-copy-template)

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

## Example SFTP Config with Group Read-Write Permissions

You can change the SFTP server settings to use a different `umask`, which in turn
affects the permissions of created files. For instance, a `umask 002` means
created files will be group read-writable.
Some SFTP servers have `umask 022` configured, meaning owner read-write only.
In the context of _ChRIS_ this might not be desirable.

```shell
[e2]
type = sftp
host = e2.tch.harvard.edu
user = chris-fnndsc
key_pem = -----BEGIN RSA PRIVATE KEY-----\nAAAAAAAAAAA==\n-----END RSA PRIVATE KEY-----\n

md5sum_command = md5sum
sha1sum_command = sha1sum
shell_type = unix
server_command = /usr/libexec/openssh/sftp-server -u 002
```
