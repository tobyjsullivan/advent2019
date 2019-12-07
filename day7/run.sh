#!/usr/bin/env bash
set -euo pipefail

COMPUTER='./target/release/day7'

build() {
  cargo build --release
}

run() {
  local acs="$1"
  local phase="$2"
  local input="$3"

  cat <<EOF | $COMPUTER "${acs}"
${phase}
${input}
EOF
}

try_perm() {
  local acs="$1"

  local phase_a="$2"
  local phase_b="$3"
  local phase_c="$4"
  local phase_d="$5"
  local phase_e="$6"

  local output_a
  local output_b
  local output_c
  local output_d
  local output_e

  output_a=$(run "${acs}" "${phase_a}" 0)
  output_b=$(run "${acs}" "${phase_b}" "${output_a}")
  output_c=$(run "${acs}" "${phase_c}" "${output_b}")
  output_d=$(run "${acs}" "${phase_d}" "${output_c}")
  output_e=$(run "${acs}" "${phase_e}" "${output_d}")

  echo "${output_e}"
}

main() {
  if [[ "$#" -lt "1" ]]; then
    echo "ERR: Must supply filename" >&2
    exit 1
  fi

  local filename="$1"

  build

  local highest
  local result
  for a in 0 1 2 3 4; do
    for b in 0 1 2 3 4; do
      if [[ "$a" -eq "$b" ]]; then
        continue
      fi
      for c in 0 1 2 3 4; do
        if [[ "$a" -eq "$c" ]] || [[ "$b" -eq "$c" ]]; then
          continue
        fi
        for d in 0 1 2 3 4; do
          if [[ "$a" -eq "$d" ]] || [[ "$b" -eq "$d" ]] || [[ "$c" -eq "$d" ]]; then
            continue
          fi
          for e in 0 1 2 3 4; do
            if [[ "$a" -eq "$e" ]] || [[ "$b" -eq "$e" ]] || [[ "$c" -eq "$e" ]] || [[ "$d" -eq "$e" ]]; then
              continue
            fi
            result=$(try_perm "${filename}" "$a" "$b" "$c" "$d" "$e")
            echo "$a $b $c $d $e - $result"
            if [[ "${result}" -gt "${highest}" ]]; then
              highest="${result}"
            fi
          done
        done
      done
    done
  done

  echo "Highest: ${highest}"
}

main "$@"
