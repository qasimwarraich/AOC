#!/bin/env bash

set -euo pipefail

FILE=input.prod

mapfile -t LEFT_LIST < <(cut -d' ' -f1 "$FILE" | sort -n)
mapfile -t RIGHT_LIST < <(cut -d' ' -f4 "$FILE" | sort -n)

DIFF_SUM=0
for i in "${!LEFT_LIST[@]}"; do
    diff=$((${LEFT_LIST[$i]} - ${RIGHT_LIST[$i]}))
    abs_diff=${diff#-}
    ((DIFF_SUM += abs_diff))
done
echo "sum of differences:" $DIFF_SUM
