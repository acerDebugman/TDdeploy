#!/bin/bash


echo ""
systemctl status taosd | grep -1 Active
echo ""
systemctl status taosx | grep -1 Active
echo ""
systemctl status taos-explorer | grep -1 Active
echo ""
systemctl status taosadapter | grep -1 Active
echo ""
systemctl status taoskeeper | grep -1 Active
