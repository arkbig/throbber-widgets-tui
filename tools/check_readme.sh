#!/bin/sh
#set -eux
set -eu

ROOTDIR=$(cd "$(dirname "$0")"/.. && pwd)

echo Check MSRV:
cargo_toml_msrv=$(grep "rust-version" "$ROOTDIR/Cargo.toml")
cargo_toml_msrv=${cargo_toml_msrv#rust-version = \"}
cargo_toml_msrv=${cargo_toml_msrv%\"}
readme_msrv=$(grep "MSRV" "$ROOTDIR/README.md")
echo "$readme_msrv" | grep "$cargo_toml_msrv"

echo Check dependencies:
cargo license --direct-deps-only --avoid-build-deps --avoid-dev-deps | awk -F ":" '{printf "|%s|%s|\n", $1, $2}' > temp.tmp
cargo license --avoid-build-deps --avoid-dev-deps | awk -F ":" '{printf "|%s|%s|\n", $1, $2}' >> temp.tmp
grep -f temp.tmp "$ROOTDIR/README.md" | diff temp.tmp -
rm temp.tmp

echo Check content between lib.rs doc comment:
awk '/\/\*!/,/\*\//' "src/lib.rs" | \diff README.md - --old-line-format='> %L' --new-line-format='< %L' --unchanged-line-format='' | diff - "$ROOTDIR/tools/readme_lib.diff"
