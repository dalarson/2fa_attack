#!/usr/bin/env bash
set -euxo pipefail

# build directory of this script
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )" NOW=$(date +%s)
BDIR="/tmp/otp-build-$NOW"

mkdir -v "$BDIR"

# build the binaries on the build container
ssh root@192.168.1.150 "cd /root/mission1/otpgen && git pull && /root/.cargo/bin/cargo build --release"

# move the binaries from the build container to our build directory
rsync -P root@192.168.1.150:/root/mission1/otpgen/target/release/attack "/tmp/attack"
rsync -P root@192.168.1.150:/root/mission1/otpgen/target/release/auth-server "/tmp/server"
rsync -P root@192.168.1.150:/root/mission1/otpgen/target/release/auth-client "/tmp/client"

# move scripts to the build
cp -v /tmp/attack         "$BDIR"
cp -v /tmp/server         "$BDIR"
cp -v /tmp/client         "$BDIR"
cp -v "$DIR/reset_arp.sh" "$BDIR"
cp -v "$DIR/spoof.sh"     "$BDIR"
cp -v "$DIR/sync_time.sh" "$BDIR"
cp -v "$DIR/ntp.conf"     "$BDIR"
cp -v "$DIR/setup.sh"    "$BDIR"

tar -czf "$BDIR"{.tar.gz,}

rm -rf "$BDIR"

