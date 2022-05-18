#!/bin/bash
# Copyright (c) Aptos
# SPDX-License-Identifier: Apache-2.0

# This script docker bake to build all the rust-based docker images
# You need to execute this from the repository root as working directory
# E.g. docker/docker-bake-rust-all.sh

set -e

## TODO(christian): add `--progress=plain` as soon as circleci supports a docker version that includes https://github.com/moby/buildkit/pull/2763
GIT_REV=$(git rev-parse --short=8 HEAD) GIT_SHA1=$(git rev-parse HEAD) BUILD_DATE="$(date -u +'%Y-%m-%dT%H:%M:%SZ')" docker buildx bake --push --progress=plain --file docker/docker-bake-rust-all.hcl $IMAGE_TARGET