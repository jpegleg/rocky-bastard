#!/bin/sh

semanage port -a -t ssh_port_t -p tcp "$1"
sed -i "s/$2/$1/g" /etc/ssh/sshd_config
grep "^Port $1" /etc/ssh/sshd_config || echo "Port $1" >> /etc/ssh/sshd_config
firewall-cmd --zone=public --permanent --add-port="$1"/tcp
firewall-cmd --reload
systemctl restart sshd
