# echo "Build"
# node ./cmd/cli.mjs compile ./test/fixtures/simple-app.js --name foo

echo "Pack"
node ./cmd/cli.mjs pack

dpkg-deb -c ./dist/foo.deb

lintian ./dist/foo.deb