#!/bin/bash

set -e
echo '=== Step 1: Running install.sh ==='
../install.sh

echo
echo '=== Step 2: Verifying installation ==='
./verify-installed.sh $1

echo
echo '=== Test completed successfully ==='