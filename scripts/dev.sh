#!/usr/bin/env bash
set -euo pipefail

export APP_MANIFEST="../clismith/Cargo.toml"
export OUTPUT_DIR="../examples/billing_api/openapi"
export PROJECT_LOCATION="$OUTPUT_DIR/app"
export BUILD_TARGET="$OUTPUT_DIR/target"
export REG_INDEX="lfs://$BUILD_TARGET/index.json"

PROJECT_NAME=test cargo run --manifest-path=$APP_MANIFEST -- "$@"

exit 0
