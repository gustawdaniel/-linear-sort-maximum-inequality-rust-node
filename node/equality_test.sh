#! /bin/sh

testEquality() {
  RES=$(cat < text | ts-node index.ts)
  EXP=$(printf "1\n2\n0")
  assertEquals "${EXP}" "${RES}"
}

# Load shUnit2.
. /usr/share/shunit2/shunit2
