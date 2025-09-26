#!/bin/bash

echo "start taosd"
systemctl start taosd 

echo "start taosadapter"
systemctl start taosadapter

echo "start taoskeeper"
systemctl start taoskeeper 

echo "start taosx"
systemctl start taosx 

echo "start taos-explorer"
systemctl start taos-explorer 

echo "done"