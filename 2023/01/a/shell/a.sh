#!/bin/env bash 

sed 's/[^0-9]//g' prod.input | while IFS= read -r line; do echo "${line:0:1}""${line: -1}"; done | paste -s -d "+" | bc
