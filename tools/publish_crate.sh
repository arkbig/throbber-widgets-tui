#!/bin/sh
# set -eux
set -eu

ROOTDIR=$(cd "$(dirname "$0")"/.. && pwd)
cd "$ROOTDIR"

crate_name=$(cargo read-manifest | jq -r ".name")
crate_version=$(cargo read-manifest | jq -r ".version")

already_exists_tag=$(gh api "repos/arkbig/$crate_name/tags" | jq "any(.[]; .name==\"v$crate_version\")")
if $already_exists_tag; then
    echo "Already exists tag v$crate_version"
    exit
fi

git tag "v$crate_version"
git push origin "v$crate_version"

cargo publish
