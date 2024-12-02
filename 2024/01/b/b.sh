#!/bin/env bash

set -euo pipefail

FILE=input.prod

mapfile -t LEFT_LIST < <(cut -d' ' -f1 "$FILE")
mapfile -t RIGHT_LIST < <(cut -d' ' -f4 "$FILE")

SIMILARITY_SCORE=0
for i in "${!LEFT_LIST[@]}"; do
    l="${LEFT_LIST[$i]}"
    SIMILARITY_COUNT=0
    for j in "${RIGHT_LIST[@]}"; do
        if [[ $l -eq $j ]]; then
            ((SIMILARITY_COUNT += 1))
        fi
    done
    if [[ SIMILARITY_COUNT -gt 0 ]]; then
        ((SIMILARITY_SCORE += SIMILARITY_COUNT * l))
    fi
done
echo "simularity score: $SIMILARITY_SCORE"
