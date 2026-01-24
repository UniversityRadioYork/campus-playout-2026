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
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.libiconv
        ];

        craneLib = crane.mkLib pkgs;
        campus-playout = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;
          buildInputs = deps;
        };
      in
      {
        packages = {
          default = campus-playout;
        };

        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            sqlx-cli
            liquidsoap
            rust-analyzer
            mold
            sqlite
          ] ++ deps;

          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath deps}";
        };
      }
    );
}

