#!/usr/bin/env bash
set -euo pipefail

if [ -z "$1" ]
  then
    echo "Please provide a release .tar.gz"
    exit 1
fi

rsync "$1" root@10.4.9.1:/root
rsync "$1" root@10.4.9.2:/root
rsync "$1" root@10.4.9.3:/root


ssh root@10.4.9.1 "tar -xf otp*.gz --strip 2"
ssh root@10.4.9.2 "tar -xf otp*.gz --strip 2"
ssh root@10.4.9.3 "tar -xf otp*.gz --strip 2"

ssh root@10.4.9.1 "/bin/bash setup.sh"
ssh root@10.4.9.2 "/bin/bash setup.sh"
ssh root@10.4.9.3 "/bin/bash setup.sh"

