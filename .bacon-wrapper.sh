#!/bin/bash

COMMAND=$1
shift

while [[ $# -gt 0 ]]; do
  case $1 in
    -y|--year)
      YEAR="$2"
      shift;;
    -d|--day)
      DAY="$2"
      YEAR=${YEAR:-2024}
      shift;;
    -*|--*)
      echo "Unknown option $1" >&2
      exit 1
      ;;
  esac
  shift
done

if [[ -n "$YEAR" ]]; then
    TEST_ARGS="y${YEAR}"
    BENCH_ARGS="${YEAR}"
fi
if [[ -n "$DAY" ]]; then
    TEST_ARGS="${TEST_ARGS}::day${DAY}::tests"
    BENCH_ARGS="${BENCH_ARGS} day ${DAY}"
fi
COLOR="--color always"

case "${COMMAND}" in
    "test")
        exec cargo test ${COLOR} -- ${COLOR} "${TEST_ARGS}"
        ;;
    "criterion")
        exec cargo bench ${COLOR} --bench solutions -- ${COLOR} "${BENCH_ARGS}"
        ;;
    *)
        echo "Unknown command" >&2
        ;;
esac
