#!/usr/bin/env bash
# execute remotely with $1 being the rocky bastard server to configure

echo "$(date +%Y-%m-%dT%H:%M:%S)Z -      starting monolith initial deployment portion - rock3 "

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - printing hashes of deployment files/ "
sha256sum files/*

echo
echo "$(date +%Y-%m-%dT%H:%M:%S)Z - verify the files are the intended versions then press the enter key to continue with the deployment"
read CONTCHECK

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - copy over motd with scp"
scp files/motd root@"$1":/etc/motd

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - copy over monolith app with scp"
scp files/monolith root@"$1":/usr/local/sbin/monolith

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - copy over pki_env app with scp"
scp files/pki_env root@"$1":/etc/monolith/pki_env

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - copy over identity.p12 app with scp"
scp files/identity.p12 root@"$1":/opt/monolith/etc/identity.p12

echo "$(date +%Y-%m-%dT%H:%M:%S)Z- copy over unit file for app with scp"
scp files/monolith.service root@"$1":/etc/systemd/system/monolith.service

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - ssh exec set DAC for monolith files"
ssh root@"$1" "chown root:root /opt/monolith/etc/identity.p12 /etc/monolith/pki_env && chmod 0600 /opt/monolith/etc/identity.p12 /etc/monolith/pki_env"

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - ssh exec set executable monolith, enable, and start the monolith"
ssh root@"$1" "chmod +x /usr/local/sbin/monolith && systemctl enable monolith && systemctl start monolith"

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - check for existing selinux file"
EXISTY=$(ssh root@$1 "ls my-monolith.pp || echo notfound" 2>/dev/null)
if [[ $EXISTY == "notfound" ]]; then
  ssh root@"$1" "ausearch -c '(monolith)' --raw | audit2allow -M my-monolith && semodule -i my-monolith.pp && /sbin/restorecon -v /usr/local/sbin/monolith"
else
  echo "$(date +%Y-%m-%dT%H:%M:%S)Z - existing selinux file found, continue..."
fi

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - restart monolith app"
ssh root@"$1" "systemctl restart monolith"

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - ensure port 443 is open in local firewall"
ssh root@"$1" "firewall-cmd --zone=public --permanent --add-port=443/tcp && firewall-cmd --reload"

echo "$(date +%Y-%m-%dT%H:%M:%S)Z - ended run of rock3 deployment script"
