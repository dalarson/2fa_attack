#!/usr/bin/env bash
set -x
ip link set arp off dev eth0
ip link set arp on dev eth0
