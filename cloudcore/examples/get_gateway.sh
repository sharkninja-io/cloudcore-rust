#!/bin/sh

netstat -rn | awk '/default/ {print $2}' | head -n 1
