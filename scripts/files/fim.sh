#!/usr/bin/env bash

f1=$(cat /dev/urandom | xxd -p | head -n2 | tail -n1 | cut -c1-8)
f2=$(cat /dev/urandom | xxd -p | head -n2 | tail -n1 | cut -c1-4)
f3=$(cat /dev/urandom | xxd -p | head -n2 | tail -n1 | cut -c1-4)
f4=$(cat /dev/urandom | xxd -p | head -n2 | tail -n1 | cut -c1-4)
f5=$(cat /dev/urandom | xxd -p | head -n2 | tail -n1 | cut -c1-12)
sessionid=$f1-$f2-$f3-$f4-$f5

echo "$(date +%Y-%m-%dT%H:%M:%SZ) - $sessionid - Looking for 'fms' program..."
fms=$(which fms || exit 1)
echo "$(date +%Y-%m-%dT%H:%M:%SZ) - $sessionid - We found $fms and will use it."
echo "$(date +%Y-%m-%dT%H:%M:%SZ) - $sessionid - SHA256 checksum of fms: $(sha256sum $fms)"
echo "$(date +%Y-%m-%dT%H:%M:%SZ) - $sessionid - reading targets from ./fimsys.sh"
source fimsys.sh

mkdir -p /opt/fimsyscheck/live /opt/fimsyscheck/previous 2>/dev/null
echo "$(date +%Y-%m-%dT%H:%M:%SZ) - $sessionid - ${fimtargets[@]}";

for x in "${fimtargets[@]}"; do
  y=$(echo $x | sha256sum | cut -c1-32)
  touch /opt/fimsyscheck/live/"$y".json
  cp /opt/fimsyscheck/live/"$y".json /opt/fimsyscheck/previous/"$y".json
  fms "$x" > /opt/fimsyscheck/live/"$y".json;
  tver=$(diff -biw /opt/fimsyscheck/live/"$y".json /opt/fimsyscheck/previous/"$y".json | sed -n 's/^> //p' | grep -v "Report time" | tr -d '\n')
  if [ "${tver[@]}" == "" ]; then
    echo "$(date +%Y-%m-%dT%H:%M:%SZ) - $sessionid - no changes for $x since last run";
  else
    logstamp=$(date +%Y-%m-%dT%H:%M:%SZ)
    echo "$logstamp - $sessionid - CHANGE DETECTED for $x - new report: $(cat /opt/fimsyscheck/live/"$y".json)";
    echo "$logstamp - $sessionid - CHANGE DETECTED for $x - old report: $(cat /opt/fimsyscheck/previous/"$y".json)";
    logger "$logstamp - $sessionid - CHANGE DETECTED for $x - new report: $(cat /opt/fimsyscheck/live/"$y".json | tr -d '\n')";
    logger "$logstamp - $sessionid - CHANGE DETECTED for $x - old report: $(cat /opt/fimsyscheck/previous/"$y".json | tr -d '\n')";
  fi
done

tar czvf /opt/fimsyscheck/event_$(date +%Y%m%dT%H%M%SZ).tgz /opt/fimsyscheck/live /opt/fimsyscheck/previous > /opt/fimsyscheck/tar.out 2>&1
find /opt/fimsyscheck/ -name "event_*.tgz" -mtime +90 -exec rm -f {} \;
