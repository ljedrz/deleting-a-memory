#!/bin/bash
# usage: sweep.sh [-j N] <model>...
#
# Every (model, step) pair is a job; they run through a worker pool. The steps of one model are
# independent - `evaluate` builds a fresh Subject per experiment - and the models are independent
# of each other, so the only thing any two jobs share is the API key. A 429 is retried with the
# server's Retry-After honoured, so throttling costs time rather than runs.
#
# Nothing is aborted early. A run that dies still writes its report, and the matrix at the end
# says which cells are missing - better than a sweep that stops at the first failure and leaves
# four models unmeasured.
set -u
here=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$here" || exit 1

jobs=8
steps="c1-attribution,c1-rest,repair,instrumented"
while [ $# -gt 0 ]; do
  case "$1" in
    -j) jobs="$2"; shift 2 ;;
    -s) steps="$2"; shift 2 ;;
    *) break ;;
  esac
done
models=("$@")
[ ${#models[@]} -eq 0 ] && { echo "usage: sweep.sh [-j N] [-s step,step] <model>..."; exit 1; }

IFS=',' read -r -a TAGS <<< "$steps"
args_for() {
  case "$1" in
    c1-attribution) echo "-e attribution -r 3" ;;
    c1-rest)        echo "-e recursion -e lie -e privilege -e feedback -r 3" ;;
    repair)         echo "-e repair" ;;
    instrumented)   echo "-e instrumented" ;;
  esac
}
export -f args_for
export here

echo "### sweep: ${#models[@]} model(s) x ${#TAGS[@]} step(s) = $(( ${#models[@]} * ${#TAGS[@]} )) jobs, $jobs at a time"
date -u +'### started %Y-%m-%dT%H%M%SZ'

for m in "${models[@]}"; do for t in "${TAGS[@]}"; do echo "$m|$t"; done; done \
| xargs -P "$jobs" -I{} bash -c '
    job="{}"; model="${job%%|*}"; tag="${job##*|}"
    echo "### start  $model :: $tag"
    bash "$here/scripts/run.sh" "$model" "$tag" $(args_for "$tag") >/dev/null 2>&1
    echo "### finish $model :: $tag"
  '

date -u +'### finished %Y-%m-%dT%H%M%SZ'
echo
echo "### results"
fail=0
for m in "${models[@]}"; do
  short="${m##*/}"
  for t in "${TAGS[@]}"; do
    last=$(ls -td "eval-runs/$short"/*-"$t" 2>/dev/null | head -1)
    if [ -z "$last" ]; then printf "  %-34s %-16s NO RUN\n" "$short" "$t"; fail=1
    elif grep -q "^  stopped:" "$last/summary.txt" 2>/dev/null; then
      printf "  %-34s %-16s STOPPED EARLY\n" "$short" "$t"; fail=1
    else
      printf "  %-34s %-16s ok  %s\n" "$short" "$t" "$(grep '^total:' "$last/summary.txt" | sed 's/total: *//')"
    fi
  done
done
echo
echo "### $([ $fail -eq 0 ] && echo 'every cell measured' || echo 'SOME CELLS MISSING - see above')"
