#!/bin/bash
set -x

#cd /root

cd taosx
git pull
cargo build
systemctl stop taosx
cp ./target/debug/taosx /usr/local/taos/bin
systemctl start taosx
#systemctl status taosx
