#!/usr/bin/env zsh

set -euo pipefail

cd "$(git rev-parse --show-toplevel)"
docker build -t edu-axum docker
