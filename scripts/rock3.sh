#!/bin/sh
# execute remotely with $1 being the rocky bastard server to configure
scp files/motd root@"$1":/etc/motd
scp files/monolith root@"$1":/usr/local/sbin/monolith
scp files/pki_env root@"$1":/etc/monolith/pki_env
scp files/identity.p12 root@"$1":/opt/monolith/etc/identity.p12
ssh root@"$1" "chown root:root /opt/monolith/etc/identity.p12 /etc/monolith/pki_env && chmod 0600 /opt/monolith/etc/identity.p12 /etc/monolith/pki_env"
ssh root@"$1" "chmod +x /usr/local/sbin/monolith && systemctl enable monolith && systemctl start monolith"
ssh root@"$1" "ausearch -c '(monolith)' --raw | audit2allow -M my-monolith && semodule -i my-monolith.pp && /sbin/restorecon -v /usr/local/sbin/monolith"
ssh root@"$1" "systemctl restart monolith"
ssh root@"$1" "firewall-cmd --zone=public --permanent --add-port=7443/tcp && firewall-cmd reload"
