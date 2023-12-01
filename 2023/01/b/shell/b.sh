#!/bin/env bash

MAP="⭐one⭐ 1
⭐two⭐ 2
⭐three⭐ 3
⭐four⭐ 4
⭐five⭐ 5
⭐six⭐ 6
⭐seven⭐ 7
⭐eight⭐ 8
⭐nine⭐ 9"

sed -E 's/(one|two|three|four|five|six|seven|eight|nine)/\1⭐\1⭐\1/' prod.input | rev | sed -E 's/(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)/\1⭐\1⭐\1/' | rev >tmp.txt
while IFS= read -r line; do
    IFS=" " read -r -a pair <<<"$line"
    sed -i "s/${pair[0]}/${pair[1]}/g" tmp.txt
done <<<"$MAP"
sed 's/[^0-9]//g' tmp.txt | while IFS= read -r line; do echo "${line:0:1}""${line: -1}"; done | paste -s -d "+" | bc
rm tmp.txt

