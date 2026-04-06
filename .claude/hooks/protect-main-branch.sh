#!/bin/bash
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$COMMAND" ]; then
  exit 0
fi

NORMALIZED=$(echo "$COMMAND" | tr '\n' ' ' | sed 's/  */ /g' | sed 's/^ *//;s/ *$//')

if echo "$NORMALIZED" | grep -qE 'git\s+push\s+.*--force|git\s+push\s+.*-f\b'; then
  echo "BLOCKED: Force-push is not allowed." >&2
  exit 2
fi

if echo "$NORMALIZED" | grep -qE 'git\s+push\s+\S+\s+(main|master)\b'; then
  echo "BLOCKED: Pushing directly to main/master is not allowed." >&2
  exit 2
fi

if echo "$NORMALIZED" | grep -qE 'git\s+merge\s+.*\b(main|master)\b'; then
  echo "BLOCKED: Merging into main/master locally is not allowed." >&2
  exit 2
fi

if echo "$NORMALIZED" | grep -qE 'git\s+(checkout|switch)\s+(main|master)\b'; then
  echo "BLOCKED: Switching to main/master is not allowed in worktree agents." >&2
  exit 2
fi

if echo "$NORMALIZED" | grep -qE 'git\s+branch\s+.*-[dD]\s+(main|master)\b'; then
  echo "BLOCKED: Deleting the main/master branch is not allowed." >&2
  exit 2
fi

exit 0
