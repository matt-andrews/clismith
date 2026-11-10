#!/usr/bin/env bash
set -euo pipefail

export APP_MANIFEST="../clismith/Cargo.toml"
export OUTPUT_DIR="../examples/billing_api/openapi"
export PROJECT_LOCATION="$OUTPUT_DIR/app"
export BUILD_TARGET="$OUTPUT_DIR/target"
export REG_INDEX="lfs://$BUILD_TARGET/index.json"

rm -rf $BUILD_TARGET
rm -rf $PROJECT_LOCATION

#PROJECT_NAME=system cargo run --manifest-path=$APP_MANIFEST -- compiler version -o table
PROJECT_NAME=system cargo run --manifest-path=$APP_MANIFEST -- compiler build --src $OUTPUT_DIR --target $BUILD_TARGET -o none
PROJECT_NAME=system cargo run --manifest-path=$APP_MANIFEST -- app-manager install test $REG_INDEX -o none
PROJECT_NAME=test cargo run --manifest-path=$APP_MANIFEST -- "$@"

exit 0
