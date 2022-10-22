#!/bin/bash

SCRIPTPATH="$( cd "$(dirname "$0")" || exit 1 >/dev/null 2>&1 ; pwd -P )"

cd "$SCRIPTPATH/../book" || exit 1

mdbook serve
