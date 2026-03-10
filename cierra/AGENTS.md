# cierra/ — Deductive Prover (Rust)

## OVERVIEW

Rust workspace implementing a deductive prover for programs annotated with Hoare-style contracts (requires/ensures/invariants). Parses Taurus language, generates verification conditions (VCs). Independent from the C++ compiler.

## STRUCTURE

```
cierra/
├── cierra/              # Main prover crate
│   ├── src/
│   │   ├── main.rs      # CLI: parse file → find function → generate VCs → print
│   │   ├── taurus.rs    # Taurus AST types and display impls
│   │   ├── vcgen.rs     # Verification condition generator
│   │   ├── subst.rs     # Substitution logic for VC generation
│   │   └── parser/      # Parser module
│   │       ├── mod.rs         # Parser entry, wraps ANTLR output
│   │       ├── cst2ast.rs     # Concrete syntax tree → AST conversion
│   │       └── taurus/        # ANTLR-generated Rust parser (DO NOT EDIT)
│   └── tests/
│       ├── parser/       # .taurus test input files
│       └── snapshots/    # insta snapshot files for regression testing
├── macro/               # Proc macro crate: `sexp!()` macro for S-expression literals
│   ├── src/             # Macro implementation
│   └── tests/           # Snapshot tests for macro expansion
├── sexp/                # Library crate: SMT-LIB v2 flavored S-expression types
│   └── src/             # Sexp data types and parsing
├── grammar/             # ANTLR4 grammar source (.g4 files)
│   └── parser/          # Taurus.g4 grammar definition
├── generate.sh          # Regenerate Rust parser from grammar (requires antlr4)
└── flake.nix            # Nix development environment
```

## WHERE TO LOOK

| Task | Files | Notes |
|------|-------|-------|
| Modify Taurus grammar | `grammar/parser/Taurus.g4` then run `./generate.sh` | Regenerates `cierra/src/parser/taurus/` |
| Add AST node type | `cierra/src/taurus.rs` | Add variant + Display impl |
| Change CST→AST conversion | `cierra/src/parser/cst2ast.rs` | Maps ANTLR parse tree to Taurus AST |
| Modify VC generation | `cierra/src/vcgen.rs` | `func_to_vc()` is the entry point |
| Add S-expression support | `sexp/src/` | SMT-LIB v2 types |
| Update macro behavior | `macro/src/` | `sexp!()` proc macro |
| Add test case | `cierra/tests/parser/` + `cargo test` | Uses insta for snapshot testing |

## CONVENTIONS

- **Nightly rustfmt**: `rustfmt.toml` requires nightly features — use `cargo +nightly fmt`
- **Snapshot testing**: Uses `insta` crate — run `cargo insta review` to update snapshots
- **Generated code**: `cierra/src/parser/taurus/` is ANTLR-generated — never hand-edit
- **Workspace crates**: Three members (`cierra`, `macro`, `sexp`) — each independently versioned
- **Nix-first dev**: `.envrc` + `flake.nix` for reproducible environment; manual setup also documented
- **License split**: GPL-3.0 for `cierra/` and `grammar/`; Apache-2.0/MIT dual-license for `sexp/` and `macro/`

## ANTI-PATTERNS

- TODOs in `taurus.rs`: Display format for True/False constants undecided (1/0 vs true/false)
- TODO in `vcgen.rs`: `func_to_vc` should return variable list but doesn't yet
- TODO in `taurus.rs`: ArithTerm may need changes for boolean logic support

## COMMANDS

```bash
# Build
cargo build

# Test (includes snapshot tests)
cargo test

# Update snapshots after intentional changes
cargo insta review

# Regenerate parser from grammar (requires ANTLR4 + JRE)
./generate.sh        # or just `generate` if using Nix

# Format (requires nightly)
cargo +nightly fmt

# Run prover
cargo run -- <file.taurus> <function_name>
```
