#!/usr/bin/env bash
set -euxo pipefail

if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root" 
   exit 1
fi

HOST=$(ip -4 addr show eth0 | grep -oP '(?<=inet\s)\d+(\.\d+){3}' | grep -o '.$')

sysctl net.ipv4.conf.all.send_redirects=0
sysctl net.ipv4.conf.all.accept_redirects=0

case $HOST in
    1) 
        echo "Configuring as host 1 (the client)"
        ;;
    2) 
	echo "Configuring as host 2 (the man in the middle)"
	apt install -y ettercap-text-only ntp
	cp ntp.conf /etc/ntp.conf
	systemctl restart ntp
        ;;
    3) 
        echo "Configuring as host 3 (the server)"
        timedatectl set-ntp 0
        ;;
    *) # anything else
        echo "nothing to do for this host" 
        ;;
esac

echo "Setup complete!"

