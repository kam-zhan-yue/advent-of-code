#!/usr/bin/env bash

declare -A matrix # associative array

rows=0
cols=0

while read line
do
  cols=0
  entry=()
  for (( i=0; i<${#line}; ++i )); do
    # echo "${line:$i:1} to ${rows}, ${cols}"
    if [[ ${rows} == 0 && ${cols} == 0 ]]; then
      echo "${line:$i:1} to ${rows}, ${cols}"
      echo "${matrix[${rows},${cols}]}"
    fi
    matrix["${rows},${cols}"]=${line:$i:1}
    # echo "${matrix[${rows},${cols}]}"
    ((cols++))
  done
  ((rows++))
done

echo "${rows}"
echo "${cols}"
for (( i=0; i<${rows}; ++i )) do
  for (( j=0; j<${cols}; ++j )) do
    printf "${matrix[${i},${j}]}"
  done
  printf "\n"
done
