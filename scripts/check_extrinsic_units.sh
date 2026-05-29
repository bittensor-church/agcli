#!/usr/bin/env bash
# Compare extrinsic unit manifest to CLI conventions and (optionally) subtensor dispatches.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== Unit safety: stake_cmds must use parse_cli_* parsers =="
if rg -q 'Balance::from_tao' src/cli/stake_cmds.rs; then
  echo "FAIL: src/cli/stake_cmds.rs uses Balance::from_tao — use parse_cli_tao_amount"
  exit 1
fi
if rg -q 'AlphaBalance::from_units|AlphaBalance::try_from_units' src/cli/stake_cmds.rs; then
  echo "FAIL: src/cli/stake_cmds.rs uses AlphaBalance parsers directly — use parse_cli_alpha_amount"
  exit 1
fi

echo "== Unit safety: manifest lib tests =="
cargo test --lib extrinsic_args --features test-utils -q

echo "== Unit safety: manifest encode regression =="
cargo test --features test-utils --test extrinsic_encoding extrinsic_manifest_all_specs_encode_against_metadata -q

DISPATCHES="$ROOT/subtensor/pallets/subtensor/src/macros/dispatches.rs"
if [[ -f "$DISPATCHES" ]]; then
  echo "== Unit safety: SubtensorModule manifest calls exist in dispatches =="
  missing=0
  while IFS= read -r call; do
    [[ -z "$call" ]] && continue
    if ! rg -q "pub fn ${call}\\(" "$DISPATCHES"; then
      echo "MISSING dispatch: SubtensorModule::$call"
      missing=1
    fi
  done < <(
    awk '
      /pallet: "SubtensorModule"/ { in_sm=1; next }
      in_sm && /call:/ {
        gsub(/.*call: "/, "")
        gsub(/".*/, "")
        print
        in_sm=0
      }
    ' src/chain/extrinsic_args.rs
  )
  if [[ "$missing" -ne 0 ]]; then
    exit 1
  fi
else
  echo "== Unit safety: subtensor submodule absent — skipping dispatch cross-check =="
fi

echo "OK: extrinsic unit manifest checks passed"
