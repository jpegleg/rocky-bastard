#!/bin/sh
# execute on a compiler node or CI node used to compile programs for the rocky bastards

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - starting builder node setup..."
echo "$(date +%Y-%m-%dT%H:%M:%S)Z - check for fips mode"
fips-mode-setup --check || exit 1

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - yum update"
yum update

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - yum upgrade"
yum upgrade

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - install aide gcc clang cmake perl perl-IPC-Cmd openssl"
yum install clamav clamav-update aide gcc clang cmake perl perl-IPC-Cmd openssl -y

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - run an aide --init"
aide --init

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - download and install matching non-FIPS version for FIPS Go 1.22.5"
wget https://go.dev/dl/go1.22.5.linux-amd64.tar.gz && tar -C /usr/local -xzf go1.22.5.linux-amd64.tar.gz

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - download Go 1.23.0 as upstream reference"
wget https://go.dev/dl/go1.23.0.linux-amd64.tar.gz

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - download FIPS Go for building/reference"
wget https://github.com/golang-fips/go/archive/refs/tags/go1.22.5-2-openssl-fips.tar.gz

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - install rustup to latest stable cargo"
rustup update || curl https://sh.rustup.rs -sSf | sh -s -- -y

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - ended rock builder node setup script"
