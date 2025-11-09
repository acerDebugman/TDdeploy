#!/bin/bash
set -x

#cd /root

cd taosx
git pull
cd explorer
sh build.sh
cd ..
cargo build -p taos-explorer
systemctl stop taos-explorer
cp ./target/debug/taos-explorer /usr/local/taos/bin
systemctl start taos-explorer

