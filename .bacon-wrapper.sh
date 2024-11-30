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
    -r|--release)
      RELEASE="--release"
      ;;
    -*|--*)
      echo "Unknown option $1" >&2
      exit 1
      ;;
  esac
  shift
done

if [[ -n "$YEAR" ]]; then
    RUN_ARGS="--year ${YEAR}"
    TEST_ARGS="y${YEAR}"
    BENCH_ARGS="${YEAR}"
fi
if [[ -n "$DAY" ]]; then
    RUN_ARGS="${RUN_ARGS} --day ${DAY}"
    TEST_ARGS="${TEST_ARGS}::day${DAY}::tests"
    BENCH_ARGS="${BENCH_ARGS} day ${DAY}"
fi
COLOR="--color always"

case "${COMMAND}" in
    "check")
        exec cargo check --all-targets ${COLOR}
        ;;
    "clippy")
        exec cargo clippy --all-targets ${COLOR}
        ;;
    "test")
        exec cargo test ${COLOR} -- ${COLOR} "${TEST_ARGS}"
        ;;
    "run")
        exec cargo run ${COLOR} ${RELEASE} -- ${COLOR} run ${RUN_ARGS}
        ;;
    "bench")
        exec cargo run ${COLOR} ${RELEASE} -- ${COLOR} bench ${RUN_ARGS}
        ;;
    "criterion")
        exec cargo bench ${COLOR} --bench solutions -- ${COLOR} "${BENCH_ARGS}"
        ;;
    *)
        echo "Unknown command" >&2
        ;;
esac
