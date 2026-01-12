#!/bin/sh

for i in $1
do
    ffmpeg -i "$i" "${i%.*}.$2"
done
