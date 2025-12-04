#!/usr/bin/env bash

# Initialise associate array (2D array hack)
declare -A matrix # associative array
rows=0
cols=0
while read line
do
  cols=0
  entry=()
  for (( i=0; i<${#line}; ++i )); do
    matrix["${rows},${cols}"]=${line:$i:1}
    ((cols++))
  done
  ((rows++))
done


directions=(
  "-1,-1",
  "-1,0",
  "-1,1",
  "0,1",
  "0,-1",
  "1,1",
  "1,0",
  "1,-1",
)

get_neighbours() {
  neighbours=0
  for (( i=0; i<${#directions[@]}; ++i )); do
    vector=${directions[i]}
    # https://stackoverflow.com/questions/918886/how-do-i-split-a-string-on-a-delimiter-in-bash
    values=(${vector//,/ })
    x=$(( ${values[0]}+${1} ))
    y=$(( ${values[1]}+${2} ))
    if [[ ${matrix[${x},${y}]} == "@" ]]; then
      ((neighbours++))
    fi
  done
  echo ${neighbours}
}

total=0
for (( i=0; i<${rows}; ++i )) do
  for (( j=0; j<${cols}; ++j )) do
    if [[ ${matrix[${i},${j}]} == "@" ]]; then
      neighbours=$(get_neighbours ${i} ${j})
      if [[ ${neighbours} < 4 ]]; then
        ((total++))
      fi
    fi
  done
done

echo "Total is ${total}"
