# Cierra

A simple deductive prover.

## Get Started

### Nix

Make sure you have [direnv](https://github.com/nix-community/nix-direnv) installed.

#### VSCode

1. Install the [Direnv](https://marketplace.visualstudio.com/items?itemName=mkhl.direnv) extension.

2. Allow `.envrc`.

#### Clion

1. Install the [Direnv integration](https://plugins.jetbrains.com/plugin/15285-direnv-integration) plugin.

2. Allow `.envrc`.

3. Restart IDE. `Import direnv` when you are prompted to.

4. Set `Language and Frameworks > Rust > Toolchain location` to `/nix/store/<blabla>`.

5. Clear caches and restart IDE. Remember to `Import direnv` each time you start the IDE.

### Manual

Ensure you have the latest stable rust toolchain.

Optionally,

- install the nightly rust toolchain because our rustfmt configuration is nightly only.
- install jre to generate grammar files.

## Development

### Regenerate grammar files

```bash
$ generate
```

or

```bash
$ ./generate.sh
```

> Please ensure the [antlr-rust fork](https://github.com/rrevenantt/antlr4rust/releases) of `antlr` is installed and available at `antlr4`.

### Format code

```bash
$ format
```

or

```bash
$ cargo +nightly fmt
```

## License

If not otherwise specified, all files in this repository are licensed under the [GPLv3](./LICENSE) license,
except for files in the following folders:

- `sexp` - [Apache-2.0](./sexp/LICENSE-APACHE) or [MIT](./sexp/LICENSE-MIT)
- `macro` - [Apache-2.0](./macro/LICENSE-APACHE) or [MIT](./macro/LICENSE-MIT)

Files under the `grammar` folder are adopted from [CMinor-Verifier](https://github.com/thufv/CMinor-Verifier) project
and licensed under [GPLv3](./grammar/COPYING).
All modifications are stated in comments of the files.