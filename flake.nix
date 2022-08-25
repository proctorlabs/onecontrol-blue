{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};

      in rec {
        # For `nix build` & `nix run`:
        defaultPackage = packages.rvlink-bridge;

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ dbus pkgconfig rustc cargo ];
        };

        packages.rvlink-bridge = naersk'.buildPackage {
          name = "rvlink-bridge";
          pname = "rvlink-bridge";
          nativeBuildInputs = with pkgs; [ pkgconfig ];
          buildInputs = with pkgs; [ dbus ];
          src = ./.;
        };

        packages.rvlink-docker = pkgs.dockerTools.buildImage {
          name = "rvlink-bridge-docker";
          tag = "latest";

          copyToRoot = [ pkgs.bash pkgs.coreutils ];

          config = {
            Cmd = [ "${defaultPackage}/bin/rvlink-bridge" ];
          };
        };
      }
    );
}
