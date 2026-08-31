#!/usr/bin/env bash
# Run a content-sufficiency attempt against the real checks.
#
# Usage: check.sh <stage-id> <sandbox-dir>
set -uo pipefail

stage="${1:?usage: check.sh <stage-id> <sandbox-dir>}"
sandbox="${2:?usage: check.sh <stage-id> <sandbox-dir>}"
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
deltaforge="${DELTAFORGE_BIN:-$repo/target/release/deltaforge}"

"$deltaforge" --project-dir "$sandbox/project" test --stage "$stage"
status=$?
echo
if [ "$status" -eq 0 ]; then
  echo "PASS  $stage passed from published content alone."
else
  echo "FAIL  $stage did not pass. The output above is what a learner would see."
fi
exit "$status"
