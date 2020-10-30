#!/bin/bash

# Copyright (c) The Libra Core Contributors
# SPDX-License-Identifier: Apache-2.0

# run this script from the project root using `./scripts/build_pydocs.sh`

if [[ "$(basename $PWD)" != "developers.libra.org" ]]; then
  echo "Didn't pass directory check."
  echo ""
  echo "The script must be run from the developers.libra.org directory via ./scripts/build_pydocs.sh"
  echo ""
  echo "You are running it from: "
  echo $(echo $(basename $PWD))
  echo ""
  exit 1
fi

# Install libra-client-sdk
echo "Installing libra-client-sdk"
python3 -m venv ./venv

./venv/bin/pip install --upgrade pip
./venv/bin/pip install libra-client-sdk pdoc3

echo "Cleaning up pydocs directory"
rm -rf pydocs
mkdir -p pydocs

echo "Generating doc from libra-client-sdk"
source ./venv/bin/activate && pdoc3 libra --html -o pydocs

rm -rf venv
