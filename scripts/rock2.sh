#!/bin/sh
# execute on the rocky bastard after enabling fips mode and rebooting

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - updating hostname from default to $1"
echo "$(date +%Y-%m-%dT%H:%M:%S)Z - do you wish to continue? press enter to continue"
read DOCONT
echo "$(date +%Y-%m-%dT%H:%M:%S)Z - changing hostname to $1"
hostname "$1" && sed -i "s/vultr/$1/g" /etc/hosts && echo "$1" > /etc/hostname

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - ensure we are in FIPS mode or exit"
fips-mode-setup --check || exit 1

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - yum update"
yum update

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - yum upgrade"
yum upgrade

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - install clamav clamav-update and aide"
yum install clamav clamav-update aide -y

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - run an aide --init"
aide --init

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - run freshclam to download latest virus patterns"
freshclam

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - make monolith app directories"
mkdir -p /opt/monolith/etc /etc/monolith

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - ended rock1 run on $1"
