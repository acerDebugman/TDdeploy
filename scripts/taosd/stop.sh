#!/bin/bash

ps -ef | grep taosd | grep -v grep | awk '{print $2}' | xargs -I{} kill {}

cnt=`ps -ef | grep xnoded | grep -v grep | wc -l`
while [ $cnt -gt 0 ]
do
        echo "kill xnode"
        ps -ef | grep xnoded | grep -v grep | awk '{print $2}' | xargs -I{} kill {}
        sleep 1
        cnt=`ps -ef | grep xnoded | grep -v grep | wc -l`
done

