#!/bin/bash

for file in *.png; do
  hash=$(md5sum "$file" | cut -d' ' -f1)
  mv "$file" "$hash.png"
done
