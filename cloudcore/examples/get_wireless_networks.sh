#!/bin/sh

networksetup -listpreferredwirelessnetworks "$1" | grep "$2"
