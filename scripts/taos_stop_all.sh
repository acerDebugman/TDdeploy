#/bin/bash

echo "stop taosd"
systemctl stop taosd 

echo "stop taosx"
systemctl stop taosx 

echo "stop taos-explorer"
systemctl stop taos-explorer 

echo "stop taosadapter"
systemctl stop taosadapter

echo "stop taoskeeper"
systemctl stop taoskeeper 

echo "done"