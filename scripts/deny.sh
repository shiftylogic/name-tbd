#!/usr/bin/env bash
#
# Copyright (c) 2022-present Robert Anderson.
# SPDX-License-Identifier: MIT
#

set -eu

SCRIPT_ROOT=`dirname $0`
LOCAL_ROOT=${SCRIPT_ROOT}/..

# Setup the local variables
CARGO_DENY_VER="0.18.5"
CARGO_DENY_REL_BASE="https://github.com/EmbarkStudios/cargo-deny/releases/download"

TARGET_TOOLS_DIR="${LOCAL_ROOT}/.tools"
TARGET_BIN="cargo-deny"

case "${OSTYPE}" in
  darwin*)
    if [[ "$(uname -m)" == "arm64" ]]
    then
      CARGO_DENY_ARCH="aarch64-apple-darwin"
    else
      CARGO_DENY_ARCH="x86_64-apple-darwin"
    fi
    ;;
  linux*)
    CARGO_DENY_ARCH="$(uname -m)-unknown-linux-musl"
    ;;
  msys*|cygwin*|win*)
    CARGO_DENY_ARCH="x86_64-pc-windows-msvc"
    TARGET_BIN="${TARGET_BIN}.exe"
    ;;
  *)
    echo "ERROR: Unsupported platform."
    ;;
esac
CARGO_DENY_REL="${CARGO_DENY_REL_BASE}/${CARGO_DENY_VER}/cargo-deny-${CARGO_DENY_VER}-${CARGO_DENY_ARCH}.tar.gz"

# Check if we already downloaded it.
if [[ ! -x "${TARGET_TOOLS_DIR}/${TARGET_BIN}" ]]
then
  echo "Downloading cargo-deny binary..."

  # Make sure the tools location already exists
  mkdir -p ${TARGET_TOOLS_DIR}

  # Download the cargo-deny pre-built
  curl -s -L ${CARGO_DENY_REL} | tar -xzf- --strip-components=1 --include="*/cargo-deny*" -C ${TARGET_TOOLS_DIR}
fi

# Now run the tool on the repo
${TARGET_TOOLS_DIR}/${TARGET_BIN} --workspace check
