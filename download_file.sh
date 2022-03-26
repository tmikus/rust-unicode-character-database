#!/usr/bin/env bash

dir="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# curl "https://www.unicode.org/Public/14.0.0/ucd/UCD.zip" -o "data/UCD.zip"
# (cd data && unzip UCD.zip)

[ -f "$dir/data/ucd.all.flat.xml" ] && exit 0

mkdir -p "$dir/data"

curl "https://www.unicode.org/Public/14.0.0/ucdxml/ucd.all.flat.zip" -o "$dir/data/ucd.zip"
(cd "$dir/data" && unzip ucd.zip)
