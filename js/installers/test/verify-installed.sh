#!/bin/sh

set -e

DIRECTORY="$(dirname "$0")"
SHELL_TO_RUN="$1"
PROFILE_FILE="$("$DIRECTORY/get_shell_profile.sh" "$SHELL_TO_RUN")"

ls -lah ~
echo "---"
echo "Shell to run: $SHELL_TO_RUN"
echo "Current Shell: $SHELL"
echo "Profile file: $PROFILE_FILE"
echo "---"
cat "$PROFILE_FILE"
echo "---"
echo "(before) PATH=$PATH"
echo "---"

$SHELL_TO_RUN -c "
  . $PROFILE_FILE
  echo \"(after) PATH=\$PATH\"
  echo "---"
  livraison --version
"
