#!/bin/sh
set -e

usage="$(basename "$0") [-h] [-c]
Start afl based project
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
  rm -rf fuzzer-afl/in/*
  mkdir -p fuzzer-afl/in/
  cp examples/* fuzzer-afl/in
  cp tests/* fuzzer-afl/in
else
  cd fuzzer-afl
  cargo afl build
  cargo afl fuzz -i in -o out -G 512 -t 10000 target/debug/afl
fi
