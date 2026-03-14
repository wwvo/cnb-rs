#!/bin/bash

set -euo pipefail

action="${1:-build-and-push}"
image="${CNB_DOCKER_REGISTRY}/${CNB_REPO_SLUG_LOWERCASE}:rust-ci"
dockerfile=".cnb/docker/rust-ci.Dockerfile"

build_image() {
  docker build -t "${image}" -f "${dockerfile}" .
}

push_image() {
  docker push "${image}"
}

case "${action}" in
  build)
    build_image
    ;;
  push)
    push_image
    ;;
  build-and-push)
    build_image
    push_image
    ;;
  *)
    echo "未知动作: ${action}" >&2
    echo "用法: build-image.sh [build|push|build-and-push]" >&2
    exit 1
    ;;
esac
