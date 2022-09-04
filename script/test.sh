#!/usr/bin/env bash
# -*- coding: utf-8 -*-

function assert() {
  diff -w ${1} ${2}
  return $?
}

COMMAND="cargo run -q --"
BASE_URL="http://www.ced.is.utsunomiya-u.ac.jp/lecture/2022/jikkenb/micro"
PROGRAMS="chap5/ex1 chap5/ex2 chap5/ex3 chap5/ex4 sample/sumof"

for PROGRAM in ${PROGRAMS}; do
  # Assemble the source program
  SOURCE=$(mktemp)
  curl -s ${BASE_URL}/${PROGRAM} | iconv -f sjis -t utf8 > ${SOURCE}
  ACTUAL=$(mktemp)
  ${COMMAND} ${SOURCE} -o ${ACTUAL}

  # Download the binary program
  EXPECTED=$(mktemp)
  curl -s ${BASE_URL}/${PROGRAM}.b | iconv -f sjis -t utf8 > ${EXPECTED}

  assert ${EXPECTED} ${ACTUAL}
  STATUS=$?
  if [ ${STATUS} -eq 0 ]; then
    echo "[SUCCESS]: $(basename $PROGRAM)"
  else
    echo "[FAILURE]: $(basename $PROGRAM)"
  fi
done
