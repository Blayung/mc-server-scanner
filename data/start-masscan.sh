#!/bin/bash
sudo masscan -p25565 0.0.0.0/0 --exclude 255.255.255.255 --rate 100000 -oL masscan-list.txt
