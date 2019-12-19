#!/usr/bin/env bash
set -euxo pipefail 

ssh root@192.168.1.150 "cd /root/mission1/otpgen && git pull && /root/.cargo/bin/cargo build"

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

rsync root@192.168.1.150:/root/mission1/otpgen/target/debug/attack ./bins/attack
rsync root@192.168.1.150:/root/mission1/otpgen/target/debug/auth-server ./bins/auth-server
rsync root@192.168.1.150:/root/mission1/otpgen/target/debug/auth-client ./bins/auth-client

rsync ./bins/auth-client root@10.4.9.1:/root/client
rsync ./bins/attack      root@10.4.9.2:/root/attack
rsync ./bins/auth-server root@10.4.9.3:/root/server



