#!/bin/sh
# execute on a compiler node or CI node used to compile programs for the rocky bastards
fips-mode-setup --check || exit 1

yum update
yum upgrade
yum install clamav clamav-update aide gcc clang cmake perl perl-IPC-Cmd openssl-y

aide --init
freshclam

wget https://go.dev/dl/go1.23.0.linux-amd64.tar.gz && tar -C /usr/local -xzf go1.23.0.linux-amd64.tar.gz
rustup update || curl https://sh.rustup.rs -sSf | sh -s -- -y
