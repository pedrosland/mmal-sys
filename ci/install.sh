#!/bin/bash

# Fetch the given release
tag="v0.1.16"
curl -LSfs https://japaric.github.io/trust/install.sh | \
  sh -s -- \
    --force \
    --git japaric/cross \
    --tag $tag
