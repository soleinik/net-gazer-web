#!/bin/bash

flatc --rust \
 -o src \
 traceroute.fbs 

flatc --rust \
 -o src \
 allipv4.fbs 




