#!/usr/bin/env bash

set -euo pipefail

target=""
profile="release"
output_dir="packaging/linux/generated"
binary_path=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --binary-path)
      binary_path="${2:?missing value for --binary-path}"
      shift 2
      ;;
    --target)
      target="${2:?missing value for --target}"
      shift 2
      ;;
    --profile)
      profile="${2:?missing value for --profile}"
      shift 2
      ;;
    --output-dir)
      output_dir="${2:?missing value for --output-dir}"
      shift 2
      ;;
    *)
      echo "unknown argument: $1" >&2
      echo "usage: $0 [--binary-path <path>] [--target <triple>] [--profile <profile>] [--output-dir <dir>]" >&2
      exit 1
      ;;
  esac
done

if [[ -n "${binary_path}" ]]; then
  :
elif [[ -n "${target}" ]]; then
  binary_path="target/${target}/${profile}/cnb-rs"
else
  binary_path="target/${profile}/cnb-rs"
fi

if [[ ! -x "${binary_path}" ]]; then
  echo "expected built binary at ${binary_path}" >&2
  exit 1
fi

rm -rf "${output_dir}"
mkdir -p "${output_dir}/bash" "${output_dir}/zsh" "${output_dir}/fish"

"${binary_path}" completion -s bash > "${output_dir}/bash/cnb-rs"
"${binary_path}" completion -s zsh > "${output_dir}/zsh/_cnb-rs"
"${binary_path}" completion -s fish > "${output_dir}/fish/cnb-rs.fish"
