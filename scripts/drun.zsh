#!/usr/bin/env zsh

set -euo pipefail

docker container run -it --network db --rm --name edu-axum -p 3000:3000 edu-axum
