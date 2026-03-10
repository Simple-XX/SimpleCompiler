# src/ — C++ Compiler Source

## OVERVIEW

Full compilation pipeline from source text to RISC-V assembly. Each subdirectory handles one pipeline stage.

## STRUCTURE

```
src/
├── main.cpp           # Entry point: orchestrates pipeline per-file
├── init.cpp           # CLI arg parsing, file list population
├── log.cpp            # spdlog initialization (SCLOG logger)
├── include/           # ALL 20 headers live here (not beside their .cpp)
├── scanner/           # scanner.cpp — file I/O, character stream
├── lexical/           # lexical.cpp + token.cpp — tokenization
├── parser/            # parser.cpp + type.cpp — recursive descent → AST
├── typechecker/       # typechecker.cpp + eval.cpp — semantic analysis
├── ir/                # irgen.cpp, irlexer.cpp, irparser.cpp, lowirgen.cpp
├── codegen/           # codegen.cpp — IR → RISC-V assembly
├── error/             # error.cpp — error reporting
└── test/              # .c test input files (not GTest)
```

## WHERE TO LOOK

| Task | Files | Notes |
|------|-------|-------|
| Add new AST node type | `include/ast.h` | Inherit `MetaAST`, implement `to_string()`, `Eval()`, `GenerateIR()` |
| Add keyword/operator | `include/token.h` (`tag_t` enum), `lexical/token.cpp` | Also update `keywords_t` constructor |
| Parser rule change | `parser/parser.cpp` | Recursive descent; ~713 lines, largest complexity |
| Type system extension | `include/type.h`, `typechecker/typechecker.cpp` | Currently int-only |
| Modify IR output | `ir/irgen.cpp` (high IR), `ir/lowirgen.cpp` (low IR) | IR is text-serialized between stages |
| Change assembly output | `codegen/codegen.cpp` | Reads low IR text, emits RISC-V |
| Add shared data struct | `include/utils.h` | `GenVar`, `Var`, `Function` structs |
| Touch logging | `log.cpp`, `include/log.h` | extern `SCLOG` logger |

## CONVENTIONS

- **Headers separate from sources**: All `.h` files in `include/`, all `.cpp` in subdirectories
- **common.h = god header**: Includes all 19 other headers + forward-declares all classes. Changing ANY header triggers full recompile
- **AST ownership**: `using ASTPtr = std::unique_ptr<MetaAST>` — move semantics throughout, explicit `.reset()` in destructors
- **3 virtual methods per AST node**: `to_string()`, `Eval(TypeCheck&)`, `GenerateIR(IRGenerator&, string&)`
- **IR serialization gap**: IRGenerator outputs a `std::string`, IRParser re-parses it via `std::istringstream` — no direct AST-to-IR pass
- **Global error pointer**: `extern Error *error` — allocated per-file in main loop, never freed via smart pointer

## COMPLEXITY HOTSPOTS

| File | Lines | Why |
|------|-------|-----|
| `typechecker/typechecker.cpp` | ~850 | Deep `FillInValue` recursion for multidimensional arrays |
| `ir/irgen.cpp` | ~788 | Exhaustive `GenerateIR()` dispatch for every AST node type |
| `parser/parser.cpp` | ~713 | Long if-else chains for token matching in recursive descent |
| `include/ast.h` | ~670 | 15+ AST node classes with repetitive structure |
| `ir/irparser.cpp` | ~700 | Re-parsing serialized IR text back into IR AST |

## ANTI-PATTERNS

- Multiple `// TODO: 只支持 int` in `parser.cpp` — type support is hardcoded to int
- `init.cpp` has a TODO suggesting CLI handling should be refactored
- AST destructors manually call `.reset()` on unique_ptrs — redundant (auto-cleanup), but consistent pattern
