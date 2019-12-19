#!/bin/bash
while true; do
  ntpdate -v -p1 10.4.9.2
  sleep 2;
done

