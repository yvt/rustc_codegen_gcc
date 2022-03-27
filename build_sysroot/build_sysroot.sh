#!/usr/bin/env bash

# Requires the CHANNEL env var to be set to `debug` or `release.`

set -e
cd $(dirname "$0")

pushd ../ >/dev/null
source ./config.sh
popd >/dev/null

# Cleanup for previous run
#     v Clean target dir except for build scripts and incremental cache
rm -r target/*/{debug,release}/{build,deps,examples,libsysroot*,native} 2>/dev/null || true
rm Cargo.lock test_target/Cargo.lock 2>/dev/null || true
rm -r sysroot/ 2>/dev/null || true

target_triple_upper="$(echo "$TARGET_TRIPLE" | tr '[:lower:]-' '[:upper:]_')"

rustflags_extra='-Z force-unstable-if-unmarked -Cpanic=abort'
if [[ "$1" == "--release" ]]; then
    rustflags_extra="$rustflags_extra -Zmir-opt-level=3"
fi

eval 'export CARGO_TARGET_${target_triple_upper}_RUSTFLAGS="$CARGO_TARGET_'${target_triple_upper}'_RUSTFLAGS $rustflags_extra"'

# Build libs
if [[ "$1" == "--release" ]]; then
    sysroot_channel='release'
    cargo build --target $TARGET_SPEC --release --verbose
else
    sysroot_channel='debug'
    cargo build --target $TARGET_SPEC --features compiler_builtins/c
fi

# Copy files to sysroot
mkdir -p sysroot/lib/rustlib/$TARGET_TRIPLE/lib/
cp -r target/$TARGET_TRIPLE/$sysroot_channel/deps/* sysroot/lib/rustlib/$TARGET_TRIPLE/lib/
