#!/usr/bin/env bash

REPO_BASE="$(git rev-parse --show-toplevel)"
HOOKS_PATH="${REPO_BASE}/.githooks"

# Set git hooks path locally
git config core.hooksPath "${HOOKS_PATH}/"

