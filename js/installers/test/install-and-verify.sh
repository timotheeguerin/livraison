#!/usr/bin/env bash

set -e

echo "Shell:"
echo "  Current: $SHELL"
echo "  Expected: $1"

echo "Setting current default shell to /bin/$1"
$SHELL=/bin/$1

../install.sh

echo
echo '=== Step 2: Verifying installation ==='
./verify-installed.sh $1

echo
echo '=== Test completed successfully ==='