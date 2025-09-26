#/bin/bash

echo "restart taosd"
systemctl restart taosd 

echo "restart taosx"
systemctl restart taosx 

echo "restart taos-explorer"
systemctl restart taos-explorer 

echo "restart taosadapter"
systemctl restart taosadapter

echo "restart taoskeeper"
systemctl restart taoskeeper 

echo "done"