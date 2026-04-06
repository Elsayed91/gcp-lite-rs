#!/bin/bash
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [ -z "$FILE_PATH" ]; then
  exit 0
fi

if echo "$FILE_PATH" | grep -qE '(/src/types/|/src/ops/|/src/test_support/)'; then
  echo "BLOCKED: Editing generated files is strictly forbidden." >&2
  echo "File: $FILE_PATH" >&2
  echo "Edit the TOML manifest in codegen/manifests/ and re-run codegen instead." >&2
  exit 2
fi

exit 0
