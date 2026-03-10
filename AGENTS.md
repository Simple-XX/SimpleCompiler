# PROJECT KNOWLEDGE BASE

**Generated:** 2026-03-10
**Commit:** ccb5391
**Branch:** main

## OVERVIEW

C++ compiler (C++20) targeting a C-like language → RISC-V assembly. Contains a separate Rust-based deductive prover (`cierra/`) for formal verification of annotated programs.

## STRUCTURE

```
SimpleCompiler/
├── src/              # C++ compiler source (pipeline: scan→lex→parse→typecheck→IR→codegen)
│   ├── include/      # 20 headers; common.h is the hub that includes everything
│   ├── codegen/      # RISC-V assembly generation
│   ├── ir/           # IR generation, parsing, low-level IR
│   ├── lexical/      # Lexer + token definitions
│   ├── parser/       # Recursive descent parser → AST
│   ├── scanner/      # File I/O scanner
│   ├── typechecker/  # Semantic analysis + type checking
│   ├── error/        # Error reporting
│   └── test/         # Test input files (.c)
├── cierra/           # Rust workspace: deductive prover (independent from C++ compiler)
├── cmake/            # CMake modules: deps (CPM), compile config, coverage functions
├── test/             # GTest unit/system tests (mostly commented out)
├── doc/              # Doxygen documentation
└── tools/            # cppcheck suppression config
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Add language construct | `src/parser/`, `src/include/ast.h`, `src/typechecker/` | Parser→AST→TypeCheck chain |
| Modify code generation | `src/codegen/codegen.cpp` | Outputs RISC-V assembly |
| Change IR representation | `src/ir/`, `src/include/ir.h`, `src/include/irast.h` | Two-level IR: high IR → low IR |
| Token/keyword changes | `src/include/token.h`, `src/lexical/` | `tag_t` enum + `keywords_t` class |
| Add type support | `src/include/type.h`, `src/typechecker/` | Currently int-only (see TODOs) |
| Build configuration | `cmake/compile_config.cmake`, `CMakePresets.json` | C17/C++20 enforced |
| Third-party deps | `cmake/3rd.cmake` | CPM-based: googletest, spdlog, PackageProject |
| Formal verification | `cierra/` | Entirely separate Rust project |
| CI pipeline | `.github/workflows/test.yml` | **Rust-only** — no C++ CI jobs |

## CONVENTIONS

- **Code style**: LLVM via `.clang-format`; extensive `.clang-tidy` checks enabled
- **Out-of-source builds only**: CMake enforces `build/` directory; in-source builds are fatal
- **Comments in Chinese**: Inline comments and CMake comments are predominantly in Chinese (中文)
- **Doxygen headers**: Every source file has a standard doxygen comment block (file, brief, author, date, copyright)
- **Header guard style**: `SIMPLECOMPILER_<FILENAME>_H` (not `#pragma once`)
- **Global state**: `Error *error` and `std::vector<std::string> src_files` are globals in `main.cpp`
- **Smart pointers for AST**: `using ASTPtr = std::unique_ptr<MetaAST>` throughout
- **Logging**: All pipeline stages log via `SPDLOG_LOGGER_INFO(SCLOG, ...)` — spdlog required

## ANTI-PATTERNS (THIS PROJECT)

- **Type system is int-only**: Multiple `// TODO: 只支持 int` in parser.cpp — do not assume multi-type support
- **Test subdirectories commented out**: `test/CMakeLists.txt` has `add_subdirectory` calls commented out
- **References SimpleRenderer**: `test/CMakeLists.txt` uses `${SimpleRenderer_SOURCE_DIR}` — likely a copy-paste bug
- **No C++ CI**: GitHub Actions workflow only builds/tests Rust (`cierra/`), not the C++ compiler

## COMMANDS

```bash
# Build C++ compiler
mkdir build && cd build
cmake --preset build ..
cmake --build .

# Run compiler
./build/bin/SimpleCompiler <input.c> -o 1    # -o 1 = lexical output
./build/bin/SimpleCompiler -h                 # help

# Code quality
make clang-format     # format all source
make clang-tidy       # lint all source
make cppcheck         # static analysis
make coverage         # test coverage report (lcov + genhtml)

# Cierra (Rust prover) — run from cierra/
cargo build
cargo test
cargo run -- <file> <function_symbol>
```

## NOTES

- Compilation pipeline in `main.cpp` is linear per-file: Scanner→Lexer→Parser→TypeCheck→IRGenerator→IRParser→LowIRGenerator→CodeGen
- IR is stringified between stages (IRGenerator outputs string, IRParser re-parses it) — not a direct AST pass
- `common.h` is a "god header" that includes all 19 other headers — any header change triggers full recompile
- The `cierra/` Rust project and C++ compiler share no code; they are co-located but independent
