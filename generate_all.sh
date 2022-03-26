#!/usr/bin/env bash

dir="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
"$dir/download_file.sh"
(cd "$dir/ucd_generate_alphabets" && cargo run)
(cd "$dir/ucd_generate_names" && cargo run)
(cd "$dir" && cargo build)
