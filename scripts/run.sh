#!/bin/bash
# usage: run.sh <model> <tag> <bench args...>
#
# One cell of a sweep. Writes eval-runs/<model>/<timestamp>-<tag>/ containing the report, the
# summary as it was printed, and a meta.json recording what produced it.
#
# The API key is read from the file named by NACHALNIK_KEY_ENV, which must live outside this
# repository and must define NACHALNIK_API_KEY and NACHALNIK_BASE_URL. It is never committed and
# never written into eval-runs.
set -u
: "${NACHALNIK_KEY_ENV:?set NACHALNIK_KEY_ENV to a file outside this repo defining NACHALNIK_API_KEY}"
. "$NACHALNIK_KEY_ENV"

here=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
instrument="$here/../nachalnik"
cd "$here" || exit 1

model="$1"; tag="$2"; shift 2
stamp=$(date -u +%Y-%m-%dT%H%M%SZ)
dir="eval-runs/${model##*/}/${stamp}-${tag}"
mkdir -p "$dir"

python3 - "$dir/meta.json" "$model" "$stamp" "$tag" "$*" "$instrument" <<'PY'
import json, re, subprocess, sys
path, model, stamp, tag, args, instrument = sys.argv[1:7]
def git(*a, cwd=None):
    return subprocess.run(["git", *a], capture_output=True, text=True, cwd=cwd).stdout.strip()
# read the version rather than hardcode it: this field said "3" through the whole of the v4
# collection, which is a provenance record disagreeing with the digests in the report beside it
src = open(f"{instrument}/nachalnik-eval/src/suite/script.rs").read()
json.dump({"model": model, "endpoint": "https://openrouter.ai/api/v1", "provider": "openrouter",
           "started": stamp, "what": tag, "bench_args": args,
           "study_commit": git("rev-parse", "HEAD"),
           "instrument_commit": git("rev-parse", "HEAD", cwd=instrument),
           "instrument_version": re.search(r'VERSION: &str = "([^"]+)"', src).group(1)},
          open(path, "w"), indent=1)
PY

cargo run -q --manifest-path "$instrument/nachalnik-eval/Cargo.toml" --example bench -- \
  -m "$model" "$@" --json "$here/$dir/report.json" 2>&1 | tee "$dir/summary.txt"
echo "EXIT=$? -> $dir"
