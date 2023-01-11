#!/usr/bin/env sh
#
# Fetch your default chrome session session cookie from your computer, and use
# that to download an input. Requires a reasonably recent session cookie (I
# think they expire after a month, so if you've logged into the website
# recently it should work).
#
# Usage:
#  ./download-input <year> <day>

set -e
set -u

PYTHON=~/.virtualenvs/pycookiecheat/bin/python

year="$1"
day="$2"
dir="$(dirname "$0")/$year/inputs/day$(printf '%02d' $day)"

session_key=$($PYTHON -c 'import pycookiecheat; print(pycookiecheat.chrome_cookies("https://adventofcode.com")["session"])')
mkdir -p "$dir"
curl -sSL -H "cookie: session=$session_key" https://adventofcode.com/$year/day/$day/input | tee "$dir/input.txt"
