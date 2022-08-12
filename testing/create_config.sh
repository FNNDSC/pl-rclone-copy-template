#!/bin/bash -e

KEY="$(awk '{printf "%s\\n", $0}')"

exec cat << EOF
[test-ssh-server]
type = sftp
host = localhost
user = chrclone-test-user
port = 2222
key_pem = $KEY
EOF
