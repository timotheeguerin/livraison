set -e

DIRECTORY="$(dirname "$0")"
SHELL_TO_RUN="$1"
PROFILE_FILE="$("$DIRECTORY/get_shell_profile.sh" "$SHELL_TO_RUN")"

echo "~/"
ls -lah ~
echo 
echo "~/.livraison"
ls -lah ~/.livraison
echo 
echo "~/.livraison/bin"
ls -lah ~/.livraison/bin

echo "---"
echo "Shell to run: $SHELL_TO_RUN"
echo "Current Shell: $SHELL"
echo "::group::Profile file: $PROFILE_FILE"
cat "$PROFILE_FILE"
echo "::endgroup::"
echo "(before) PATH=$PATH"
echo "---"

$SHELL_TO_RUN -c "
  . $PROFILE_FILE && echo \"(after) PATH=\$PATH\" && livraison --version
"
