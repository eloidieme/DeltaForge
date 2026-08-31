#!/usr/bin/env bash
# Prepare one content-sufficiency attempt.
#
# Writes a sandbox containing exactly two things: the content a learner can see
# for a stage, and a project to implement it in. The sandbox holds no tests, no
# fixtures, and no reference solution, so an attempt made inside it can only
# draw on published content.
#
# Usage: prepare.sh <stage-id> <sandbox-dir>
set -euo pipefail

stage="${1:?usage: prepare.sh <stage-id> <sandbox-dir>}"
sandbox="${2:?usage: prepare.sh <stage-id> <sandbox-dir>}"
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
deltaforge="${DELTAFORGE_BIN:-$repo/target/release/deltaforge}"

if [ ! -x "$deltaforge" ]; then
  echo "build the release binary first: cargo build --release" >&2
  exit 1
fi

rm -rf "$sandbox"
mkdir -p "$sandbox"

# Everything the learner has read by the time they reach this stage: this
# stage's guide and every earlier one, help ladders included.
: > "$sandbox/CONTENT.md"
for earlier in $("$deltaforge" pack content flashindex --json \
  | python3 -c 'import json,sys
stages=[s["stage_id"] for s in json.load(sys.stdin)]
print(" ".join(stages[: stages.index(sys.argv[1]) + 1]))' "$stage"); do
  "$deltaforge" pack content flashindex --stage "$earlier" >> "$sandbox/CONTENT.md"
  printf '\n---\n\n' >> "$sandbox/CONTENT.md"
done

"$deltaforge" init flashindex --lang rust --stage "$stage" \
  --name "$sandbox/project" --no-git >/dev/null

# The project ships with the pack's starter template; the attempt replaces
# src/main.rs. Remove anything that could leak the contract.
rm -rf "$sandbox/project/.deltaforge/design_notes"

cat > "$sandbox/README.md" <<EOF
# Content-sufficiency attempt: $stage

Read CONTENT.md. Write project/src/main.rs so the program it describes behaves
as specified. Do not consult anything outside this directory.

Check your work with:

    $repo/tools/content_sufficiency/check.sh $stage $sandbox
EOF

echo "$sandbox"
