#!/bin/bash
# Copyright (c) The Aptos Core Contributors
# SPDX-License-Identifier: Apache-2.0
set -e

DIR="$( cd "$( dirname "$0" )" && pwd )"

$DIR/../aptos-build.sh $DIR/Dockerfile aptos/init "$@"
