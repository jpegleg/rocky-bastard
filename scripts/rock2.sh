#!/bin/sh
# execute on the rocky bastard after enabling fips mode and rebooting

hostname "$1" && sed -i "s/vultr/$1/g" /etc/hosts && echo "$1" > /etc/hostname

fips-mode-setup --check || exit 1

yum update
yum upgrade
yum install clamav clamav-update aide -y

aide --init
freshclam

mkdir -p /opt/monolith/etc /etc/monolith
