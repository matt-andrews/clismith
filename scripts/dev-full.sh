#!/usr/bin/env bash
set -euo pipefail

export APP_MANIFEST="../clismith/Cargo.toml"
export OUTPUT_DIR="../examples/billing_api/openapi"
export PROJECT_LOCATION="$OUTPUT_DIR/app"
export BUILD_TARGET="$OUTPUT_DIR/target"
export REG_INDEX="lfs://$BUILD_TARGET/index.json"

rm -rf $BUILD_TARGET
rm -rf $PROJECT_LOCATION

cargo run --manifest-path=$APP_MANIFEST --bin clismith-builder -- build --src $OUTPUT_DIR --target $BUILD_TARGET
cargo run --manifest-path=$APP_MANIFEST --bin clismith-runtime -- app-install $REG_INDEX
cargo run --manifest-path=$APP_MANIFEST --bin clismith-runtime -- "$@"

exit 0
