#!/bin/sh

set -e

usage="$(basename "$0") [-h] [-c]
Start libfuzzer based project
where:
    -h  show this help text
    -c  clean input corpus"

options=':hc'
while getopts $options option; do
  case "$option" in
    h) echo "$usage"; exit;;
    c) CLEAN=1;;
    :) printf "missing argument for -%s\n" "$OPTARG" >&2; echo "$usage" >&2; exit 1;;
   \?) printf "illegal option: -%s\n" "$OPTARG" >&2; echo "$usage" >&2; exit 1;;
  esac
done


if [ "$CLEAN" ] ; then
    echo "Cleaning input corpus"
    rm -rf fuzzer-libfuzzer/corpus/fuzz_target_1/*
    cp tests/* fuzzer-libfuzzer/corpus/fuzz_target_1
    cp examples/* fuzzer-libfuzzer/corpus/fuzz_target_1
else
    # cargo fuzz list --fuzz-dir ./fuzzer-libfuzzer/
    cargo fuzz build  --fuzz-dir fuzzer-libfuzzer
    cargo fuzz run fuzz_target_1 --fuzz-dir ./fuzzer-libfuzzer/  -- -max_len=512 -timeout=3
fi
