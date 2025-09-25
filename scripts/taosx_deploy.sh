#!/bin/bash

# make sure home is parent
# wd=$(pwd)

fpath=../taosx
tpath=/usr/local/taos

systemctl stop taosx
cp $fpath/target/debug/taosx $tpath/bin/taosx


systemctl start taosx
echo "done"