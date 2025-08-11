set -e

echo "Shell:"
echo "  Current: $SHELL"
echo "  Expected: $1"

../install.sh

echo
echo '=== Step 2: Verifying installation ==='
./verify-installed.sh $1

echo
echo '=== Test completed successfully ==='