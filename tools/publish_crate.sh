#!/bin/bash
# set -eux
set -eu

ROOTDIR=$(cd "$(dirname "$0")"/.. && pwd)

crate_version=$(cd "$ROOTDIR" && cargo read-manifest | jq -r ".version")

already_exists_tag=$(gh api repos/arkbig/throbber-widgets-tui/tags | jq "any(.[]; .name==\"v$crate_version\")")
if $already_exists_tag; then
    echo "Already exists tag v$crate_version"
    exit
fi

pushd "$ROOTDIR"
git tag "v$crate_version"
git push origin "v$crate_version"

echo \$CRATE_IO_TOKEN | envsubst | cargo login
cargo publish
popd
