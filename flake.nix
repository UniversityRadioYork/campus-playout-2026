{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils }:
    flake-utils.lib.eachDefaultSystem(system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        deps = with pkgs; [
          mold
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.libiconv
        ];

        craneLib = crane.mkLib pkgs;
        campus-playout = craneLib.buildPackage {
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type: (craneLib.filterCargoSources path type)
              || (builtins.match ".*/src/assets/.*$" path != null)
              || (builtins.match ".*/migrations/.*\\.sql$" path != null)
              || (builtins.match ".*/.sqlx/.*\\.json$" path != null);
            name = "source";
          };
          buildInputs = deps;
        };
        campus-playout-streamer = pkgs.stdenv.mkDerivation {
          pname = "campus-playout-streamer";
          version = "0.1.0";
          src = ./scripts;
          nativeBuildInputs = [ pkgs.makeWrapper ];
          dontBuild = true;
          installPhase = ''
            runHook preInstall

            mkdir -p $out/{bin,libexec}
            cp $src/*.liq $out/libexec

            makeWrapper ${pkgs.lib.getExe pkgs.liquidsoap} $out/bin/campus-playout-streamer \
              --add-flags $out/libexec/playout.liq
          '';
        };
      in
      {
        packages = {
          default = campus-playout;
          inherit campus-playout campus-playout-streamer;
          docker = pkgs.callPackage ./docker.nix {
            inherit campus-playout campus-playout-streamer;
          };
        };

        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            sqlx-cli
            liquidsoap
            rust-analyzer
            sqlite
          ] ++ deps;

          shellHook = ''
            if [ -f .env ]; then
              set -a
              source .env
              set +a
            fi
          '';

          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath deps}";
        };
      }
    );
}
