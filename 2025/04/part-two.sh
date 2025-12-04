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
    # https://stackoverflow.com/questions/918886/how-do-i-split-a-string-on-a-delimiter-in-bash
    vector=(${directions[i]//,/ })
    x=$(( ${vector[0]}+${1} ))
    y=$(( ${vector[1]}+${2} ))
    if [[ ${matrix[${x},${y}]} == "@" ]]; then
      ((neighbours++))
    fi
  done
  echo ${neighbours}
}

remove=()
removed=0

while : ; do
  # Get all locations that need to be removed
  for (( i=0; i<${rows}; ++i )) do
    for (( j=0; j<${cols}; ++j )) do
      if [[ ${matrix[${i},${j}]} == "@" ]]; then
        neighbours=$(get_neighbours ${i} ${j})
        if [[ ${neighbours} < 4 ]]; then
          remove+=("${i},${j}")
          ((removed++))
        fi
      fi
    done
  done

  # Check if we can't remove anymore and break
  if [[ ${#remove[@]} == 0 ]]; then
    break
  fi

  # Remove Rolls
  for (( i=0; i<${#remove[@]}; ++i )); do
    matrix["${remove[i]}"]="x"
  done

  unset remove
done

echo "Total Removed is ${removed}"
