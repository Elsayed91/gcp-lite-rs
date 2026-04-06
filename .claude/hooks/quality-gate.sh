#!/bin/bash
if ! cargo fmt --check >/dev/null 2>&1; then
  echo "Quality gate FAILED: formatting issues." >&2
  exit 2
fi

CLIPPY_OUTPUT=$(cargo clippy -- -D warnings 2>&1)
if [ $? -ne 0 ]; then
  echo "Quality gate FAILED: clippy warnings." >&2
  echo "$CLIPPY_OUTPUT" | tail -30 >&2
  exit 2
fi

TEST_OUTPUT=$(cargo test --lib 2>&1)
if [ $? -ne 0 ]; then
  echo "Quality gate FAILED: tests failing." >&2
  echo "$TEST_OUTPUT" | grep -E "FAILED|panicked|failures:|test result:" | head -20 >&2
  exit 2
fi

exit 0
