#!/bin/sh
set -e

usage="$(basename "$0") [-h] [-c]
Start honggfuzz based project
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

export HFUZZ_RUN_ARGS="-t 5 -T"

if [ "$CLEAN" ] ; then
  echo "Cleaning input corpus"
  rm -rf fuzzer-honggfuzz/hfuzz_workspace/fuzzer-honggfuzz/input/*
  mkdir -p fuzzer-honggfuzz/hfuzz_workspace/fuzzer-honggfuzz/input
  cp examples/* fuzzer-honggfuzz/hfuzz_workspace/fuzzer-honggfuzz/input/
  cp tests/* fuzzer-honggfuzz/hfuzz_workspace/fuzzer-honggfuzz/input/
else
  cd fuzzer-honggfuzz
  cargo hfuzz build
  cargo hfuzz run fuzzer-honggfuzz
fi
