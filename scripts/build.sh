#!/usr/bin/env bash
set -euo pipefail

# Simple build script to produce a wasm artifact and (optionally) JS bindings.
# Behavior:
# - Ensures the Rust target is installed (default: wasm32-unknown-unknown)
# - Runs `cargo build --release --target $TARGET`
# - If `wasm-pack` is available, runs `wasm-pack build`
# - Else if `wasm-bindgen` is available, runs `wasm-bindgen` on the produced .wasm
# - Otherwise copies the produced .wasm into the output dir and prints next steps

TARGET="${TARGET:-wasm32-unknown-unknown}"
OUT_DIR="${OUT_DIR:-pkg}"

echo "Target: ${TARGET}"
echo "Output dir: ${OUT_DIR}"

command -v cargo >/dev/null 2>&1 || { echo "cargo not found in PATH" >&2; exit 1; }

# Add the target if missing
if command -v rustup >/dev/null 2>&1; then
	if ! rustup target list --installed | grep -qx "${TARGET}"; then
		echo "Adding Rust target ${TARGET} via rustup..."
		rustup target add "${TARGET}"
	fi
else
	echo "rustup not found; assuming target ${TARGET} is already installed or managed externally"
fi

echo "Building release for ${TARGET}..."
cargo build --release --target "${TARGET}"

# locate the produced .wasm
wasm_file=$(find target/"${TARGET}"/release -maxdepth 1 -name '*.wasm' -printf '%p\n' | sort -u | head -n1 || true)
if [[ -z "${wasm_file}" ]]; then
	echo "No .wasm found under target/${TARGET}/release" >&2
	exit 1
fi

echo "Found wasm: ${wasm_file}"

if command -v wasm-pack >/dev/null 2>&1; then
	echo "Found wasm-pack; running wasm-pack build (disabling default features, enabling wasm feature)"
	# wasm-pack will run cargo itself; pass cargo flags after --
	wasm-pack build --release --target bundler --out-dir "${OUT_DIR}" -- --no-default-features --features wasm
	echo "wasm-pack output: ${OUT_DIR}/"
	exit 0
fi

if command -v wasm-bindgen >/dev/null 2>&1; then
	echo "Found wasm-bindgen; generating JS bindings into ${OUT_DIR}/"
	mkdir -p "${OUT_DIR}"
	wasm-bindgen "${wasm_file}" --out-dir "${OUT_DIR}" --target bundler
	echo "wasm-bindgen output: ${OUT_DIR}/"
	exit 0
fi

echo "Neither wasm-pack nor wasm-bindgen found. Copying raw .wasm to ${OUT_DIR}/"
mkdir -p "${OUT_DIR}"
cp "${wasm_file}" "${OUT_DIR}/"
echo "WASM copied to ${OUT_DIR}/$(basename "${wasm_file}")"
echo "To generate JS bindings, install either:\n  - wasm-pack (https://rustwasm.github.io/wasm-pack/installer/)\n  - or wasm-bindgen-cli (cargo install wasm-bindgen-cli)"

echo "Done."
