#!/bin/bash -e

KEY="$(awk '{printf "%s\\n", $0}')"

exec cat << EOF
[test-ssh-server]
type = sftp
host = localhost
user = chrclone-test-user
port = 2222
key_pem = $KEY
shell_type = unix
md5sum_command = md5sum
sha1sum_command = sha1sum
EOF
