#!/usr/bin/env bash

set -euo pipefail

default_version="v0.11.1"
default_source="${CNB_RS_INSTALL_SOURCE:-cnb}"
cnb_web_endpoint="${CNB_RS_INSTALL_CNB_WEB_ENDPOINT:-https://cnb.cool}"
cnb_repo_slug="${CNB_RS_INSTALL_REPO_SLUG:-wwvo/cnb-rs/cnb-rs}"
github_web_endpoint="${CNB_RS_INSTALL_GITHUB_WEB_ENDPOINT:-https://github.com}"
github_repo_slug="${CNB_RS_INSTALL_GITHUB_REPO_SLUG:-wwvo/cnb-rs}"
version=""
download_source=""
bin_dir=""

usage() {
  cat <<EOF
Usage: install.sh [--version <version>] [--source <cnb|github>] [--bin-dir <dir>] [--help]

Installs cnb-rs from the default CNB Release mirror or GitHub Releases.

Options:
  --version <version>  Install a specific release tag, such as v0.10.2 or 0.10.2
  --source <source>    Release source to use: cnb or github
  --bin-dir <dir>      Install directory for the cnb-rs binary
  --help               Show this help message

If --version is omitted, the installer uses the bundled default release: ${default_version}
If --source is omitted, the installer uses the configured default source: ${default_source}
EOF
}

log() {
  printf '[cnb-rs installer] %s\n' "$*"
}

fail() {
  printf '[cnb-rs installer] %s\n' "$*" >&2
  exit 1
}

have_command() {
  command -v "$1" >/dev/null 2>&1
}

download_file() {
  local url="$1"
  local destination="$2"

  if have_command curl; then
    curl -fsSL \
      -H "User-Agent: cnb-rs-install.sh" \
      -H "Accept: application/octet-stream" \
      -o "$destination" \
      "$url"
    return
  fi

  if have_command wget; then
    wget -qO "$destination" \
      --header="User-Agent: cnb-rs-install.sh" \
      --header="Accept: application/octet-stream" \
      "$url"
    return
  fi

  fail "curl or wget is required to download release assets"
}

normalize_version() {
  local raw_version="$1"

  if [[ -z "$raw_version" ]]; then
    fail "version must not be empty"
  fi

  if [[ "$raw_version" == v* ]]; then
    printf '%s\n' "$raw_version"
  else
    printf 'v%s\n' "$raw_version"
  fi
}

to_lower() {
  printf '%s' "$1" | tr '[:upper:]' '[:lower:]'
}

normalize_source() {
  local raw_source="$1"
  local normalized_source=""

  if [[ -z "$raw_source" ]]; then
    fail "download source must not be empty"
  fi

  normalized_source="$(to_lower "$raw_source")"
  case "$normalized_source" in
    cnb|github)
      printf '%s\n' "$normalized_source"
      ;;
    *)
      fail "unsupported download source: ${raw_source} (expected: cnb or github)"
      ;;
  esac
}

build_release_base_url() {
  local selected_source="$1"
  local selected_version="$2"

  case "$selected_source" in
    cnb)
      printf '%s/%s/-/releases/download/%s\n' "$cnb_web_endpoint" "$cnb_repo_slug" "$selected_version"
      ;;
    github)
      printf '%s/%s/releases/download/%s\n' "$github_web_endpoint" "$github_repo_slug" "$selected_version"
      ;;
    *)
      fail "unsupported download source: ${selected_source}"
      ;;
  esac
}

default_bin_dir() {
  if [[ "$(id -u)" -eq 0 ]]; then
    printf '/usr/local/bin\n'
    return
  fi

  if [[ -d /usr/local/bin && -w /usr/local/bin ]]; then
    printf '/usr/local/bin\n'
    return
  fi

  printf '%s/.local/bin\n' "${HOME:?HOME is required when /usr/local/bin is not writable}"
}

verify_checksum() {
  local archive_path="$1"
  local checksum_path="$2"
  local asset_name="$3"
  local expected_checksum=""
  local actual_checksum=""

  if have_command sha256sum; then
    actual_checksum="$(sha256sum "$archive_path" | awk '{print $1}')"
  elif have_command shasum; then
    actual_checksum="$(shasum -a 256 "$archive_path" | awk '{print $1}')"
  elif have_command openssl; then
    actual_checksum="$(openssl dgst -sha256 "$archive_path" | awk '{print $NF}')"
  else
    log "Skipping checksum verification because no SHA-256 tool is available"
    return
  fi

  expected_checksum="$(awk -v name="$asset_name" '$2 == name { print $1 }' "$checksum_path" | head -n 1)"

  if [[ -z "$expected_checksum" ]]; then
    fail "failed to find a checksum entry for ${asset_name}"
  fi

  if [[ "$(to_lower "$actual_checksum")" != "$(to_lower "$expected_checksum")" ]]; then
    fail "checksum verification failed for ${asset_name}"
  fi

  log "Verified SHA-256 checksum for ${asset_name}"
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      version="$(normalize_version "${2:?missing value for --version}")"
      shift 2
      ;;
    --source)
      download_source="$(normalize_source "${2:?missing value for --source}")"
      shift 2
      ;;
    --bin-dir)
      bin_dir="${2:?missing value for --bin-dir}"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      fail "unknown argument: $1"
      ;;
  esac
done

if [[ -z "$version" ]]; then
  version="$default_version"
  log "Using bundled default release version ${version}"
fi

if [[ -z "$download_source" ]]; then
  download_source="$(normalize_source "$default_source")"
fi

log "Using download source ${download_source}"

os_name="$(uname -s)"
arch_name="$(uname -m)"
target=""

case "$os_name" in
  Linux)
    case "$arch_name" in
      x86_64|amd64)
        target="x86_64-unknown-linux-musl"
        ;;
      aarch64|arm64)
        target="aarch64-unknown-linux-musl"
        ;;
      *)
        fail "unsupported Linux architecture: ${arch_name}"
        ;;
    esac
    ;;
  Darwin)
    case "$arch_name" in
      x86_64|amd64)
        target="x86_64-apple-darwin"
        ;;
      aarch64|arm64)
        target="aarch64-apple-darwin"
        ;;
      *)
        fail "unsupported macOS architecture: ${arch_name}"
        ;;
    esac
    ;;
  MINGW*|MSYS*|CYGWIN*)
    fail "Windows is not supported by install.sh; use install.ps1 instead"
    ;;
  *)
    fail "unsupported operating system: ${os_name}"
    ;;
esac

if [[ -z "$bin_dir" ]]; then
  bin_dir="$(default_bin_dir)"
fi

asset_name="cnb-rs-${version}-${target}.tar.gz"
release_base_url="$(build_release_base_url "$download_source" "$version")"
archive_url="${release_base_url}/${asset_name}"
checksum_url="${release_base_url}/sha256sum.txt"

tmp_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

archive_path="${tmp_dir}/${asset_name}"
checksum_path="${tmp_dir}/sha256sum.txt"
extract_dir="${tmp_dir}/extract"

log "Downloading ${asset_name}"
download_file "$archive_url" "$archive_path"

log "Downloading sha256sum.txt"
download_file "$checksum_url" "$checksum_path"
verify_checksum "$archive_path" "$checksum_path" "$asset_name"

mkdir -p "$extract_dir"
tar -xzf "$archive_path" -C "$extract_dir"

binary_path="${extract_dir}/cnb-rs-${version}-${target}/cnb-rs"
if [[ ! -f "$binary_path" ]]; then
  binary_path="$(find "$extract_dir" -type f -name cnb-rs | head -n 1)"
fi

if [[ -z "${binary_path:-}" || ! -f "$binary_path" ]]; then
  fail "failed to locate cnb-rs in the extracted archive"
fi

mkdir -p "$bin_dir"
bin_dir="$(cd "$bin_dir" && pwd -P)"
destination="${bin_dir}/cnb-rs"

if have_command install; then
  install -m 755 "$binary_path" "$destination"
else
  cp "$binary_path" "$destination"
  chmod 755 "$destination"
fi

installed_version="$("$destination" --version 2>/dev/null | head -n 1 || true)"
if [[ -n "$installed_version" ]]; then
  log "Installed ${installed_version} to ${destination}"
else
  log "Installed cnb-rs to ${destination}"
fi

case ":${PATH}:" in
  *":${bin_dir}:"*)
    ;;
  *)
    log "Add ${bin_dir} to PATH if cnb-rs is not yet available in your shell"
    ;;
esac
