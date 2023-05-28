#!/usr/bin/env bash

if [[ ! "$OSTYPE" =~ "darwin" ]]; then
  printf "Error: Script requires a darwin operating system.\n"
  exit 1
fi

if [[ $(/usr/bin/id -u) != 0 ]]; then
  printf "Error: Script requires root privilege.\n"
  exit 1
fi

printf "This script will purge your disk cache. Continue? [y/n]: "

read -r proceed

if [[ "$proceed" != "y" ]]; then
  echo "Aborted."
  exit 0
fi

cargo build --release

# Clear disk cache
purge

fifo="$TMPDIR"
fifo+="erd_performance"

if [[ -f "$fifo" ]]; then
  rm "$fifo"
fi

exec 3<>$fifo

trap "rm -f $fifo" SIGINT

iostat_output=
while read -r line; do
  iostat_output+="$line\n"

  read -r -u3 -t1 finished

  if [[ "$finished" == "1" ]]; then
    printf "$iostat_output"
    rm "$fifo"
    exit 0
  fi
done < <(iostat -w1) &

iostat_job="$!"

trap "kill $iostat_job 2> /dev/null" SIGINT

echo "Executing command: target/release/erd ${@}"
echo

/usr/bin/time -p target/release/erd "$@" 1> /dev/null

echo

echo "1" >> "$fifo"

wait "$iostat_job"
