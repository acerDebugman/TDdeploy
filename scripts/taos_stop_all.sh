#/bin/bash

echo "stop taos-explorer"
systemctl stop taos-explorer 

echo "stop taosx"
systemctl stop taosx 

echo "stop taosadapter"
systemctl stop taosadapter

echo "stop taoskeeper"
systemctl stop taoskeeper 

echo "stop taosd"
systemctl stop taosd 


echo "done"