#!/bin/bash
step=10

for i in {1..5};
do
        curl -s http://127.0.0.1:9003 >> /dev/null

        if [ $? != 0 ];then
                cd /opt/fixed/ && nohup /opt/fixed/wx > log.txt&
        fi
        sleep $step
done
