#!/bin/bash


ps -ef | grep taosd | grep -v grep | awk '{print $2}' | xargs -I{} kill {}
rm 1.log
rm /var/log/taos/taosdlog.0
rm -rf /var/lib/taos/*
#/app/TDengine/debug/build/bin/taosd 
#/app/TDengine/debug/build/bin/taosd > 1.log 2>&1 &
