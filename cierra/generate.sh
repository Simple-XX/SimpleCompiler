#!/usr/bin/env bash
set -euo pipefail

function to_rust_path() {
  local path=$1

  local relative_path=${path#grammar/}
  local relative_parent_path=${relative_path%/*}
  local base_name=${relative_path##*/}
  local base_name_without_extension=${base_name%.*}
  local lower_base_name_without_extension=${base_name_without_extension,,}

  local rust_path="src/${relative_parent_path}/${lower_base_name_without_extension}"
  echo "$rust_path"
}

grammars=(grammar/**/*.g4)
echo "Generating Rust files from ${#grammars[@]} grammars..."
for grammar in "${grammars[@]}"; do
  rust_path=$(to_rust_path "$grammar")
  echo "Generating $rust_path from $grammar"

  abs_rust_path="$(pwd)/$rust_path"
  grammar_name="${grammar##*/}"

  mkdir -p "$rust_path"
  pushd "${grammar%/*}" >/dev/null
  antlr4 -Dlanguage=Rust -o "$abs_rust_path" "$grammar_name"
  popd >/dev/null
done
