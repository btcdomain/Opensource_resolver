#!/bin/bash
#set -x

proc_name="btcdomain_resolver"

function start(){
  echo `date +'%Y-%m-%d %H:%M:%S'` "stop ${proc_name}"
  kill `ps -u $USER -f | grep ${proc_name} | grep -v grep | awk '{print $2}'` 2> /dev/null
  stopTryNum=1
  while [ -n "`ps -u $USER -f | grep ${proc_name} | grep -v grep`" -a $stopTryNum -lt 20 ]
  do
      echo `date +'%Y-%m-%d %H:%M:%S'` " wait for ${proc_name} dying out,  try num: $stopTryNum"
      let stopTryNum++
      sleep 10
  done
  mkdir -p ~/logs

  echo `date +'%Y-%m-%d %H:%M:%S'` "start ${proc_name}"
  setsid ~/server/${proc_name} > ~/logs/${proc_name}.`date +"%Y%m%d%H%M"`.log 2>&1 &
  sleep 3
}

function stop(){
  echo `date +'%Y-%m-%d %H:%M:%S'` "stop ${proc_name}"
  kill `ps -u $USER -f | grep ${proc_name} | grep -v grep | awk '{print $2}'` 2> /dev/null
  stopTryNum=1
  while [ -n "`ps -u $USER -f | grep ${proc_name} | grep -v grep`" -a $stopTryNum -lt 20 ]
  do
      echo `date +'%Y-%m-%d %H:%M:%S'` " wait for ${proc_name} dying out,  try num: $stopTryNum"
      let stopTryNum++
      sleep 10
  done
}

case "$1" in
  start)
        start
        ;;
  stop)
        stop
        ;;
  restart)
        stop
        start
        ;;
  *)
        echo "Usage: ~/server/control_xxx.sh {start|stop|restart}" || true
        exit 1
esac
exit 0
