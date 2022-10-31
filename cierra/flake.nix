{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    antlr4-rust-unwrapped = {
      url = "https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar";
      flake = false;
    };
  };

  outputs = { self, flake-utils, naersk, nixpkgs, rust-overlay, antlr4-rust-unwrapped }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        naersk' = pkgs.callPackage naersk { };

        rustMinimal = pkgs.rust-bin.stable.latest.default;
        rustStable = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };
        rustNightly = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.minimal.override {
          extensions = [ "rustfmt" ];
        });

        generateScript = builtins.readFile ./generate.sh;
        format = pkgs.writeShellApplication {
          name = "format";
          runtimeInputs = [ rustNightly ];
          text = ''
            cargo fmt "$@"
          '';
        };
        antlr4-rust = pkgs.stdenv.mkDerivation {
          name = "antlr4-rust";
          pname = "antlr";
          src = antlr4-rust-unwrapped;
          dontUnpack = true;
          nativeBuildInputs = [ pkgs.makeWrapper ];
          installPhase = ''
            runHook preInstall

            jar_name="''${src##*/}"

            mkdir -p "$out"/{share/java,bin}
            cp "$src" "$out/share/java/$jar_name"
            makeWrapper ${pkgs.jre}/bin/java "$out/bin/antlr" \
              --add-flags "-cp $out/share/java/$jar_name org.antlr.v4.Tool" \
              --set _JAVA_OPTIONS '-Xmx500M'
            makeWrapper ${pkgs.jre}/bin/java "$out/bin/grun" \
              --add-flags "-cp $out/share/java/$jar_name org.antlr.v4.gui.TestRig"
            ln -s "$out/bin/antlr"{,4}

            runHook postInstall
          '';
        };
        generate = pkgs.writeShellApplication {
          name = "generate";
          runtimeInputs = [ antlr4-rust ];
          text = generateScript;
        };

        darwinBuildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.darwin.libiconv pkgs.darwin.Security ];
        additionalBuildInputs = with pkgs; [ jre ] ++ darwinBuildInputs;
        devBuildInputs = [ antlr4-rust rustStable pkgs.rust-analyzer pkgs.shellcheck format generate ];

      in
      rec {
        # For `nix build` & `nix run`:
        packages = rec {
          cierra = naersk'.buildPackage rec {
            src = ./.;
            cargo = rustMinimal;
            rustc = rustMinimal;
            nativeBuildInputs = additionalBuildInputs;
            override = _: {
              preBuild = ''
                cp -r ${src}/grammar .
                ${pkgs.lib.getExe generate}
              '';
            };
          };
          default = cierra;
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = devBuildInputs ++ additionalBuildInputs;
        };

        apps = rec {
          default = cierra;
          cierra = flake-utils.lib.mkApp {
            drv = self.packages.${system}.cierra;
          };
        };
      }
    );
}
