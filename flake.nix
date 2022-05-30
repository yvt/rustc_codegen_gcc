{
  description = "Rust GCC codegen";

  inputs = {
    libgccjit-gcc.url = "github:yvt/nix-rustc_codegen_gcc";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, libgccjit-gcc, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) { inherit system; };
        inherit (pkgs) runCommand symlinkJoin;
        inherit (libgccjit-gcc.packages.${system})
          rx-embedded-gcc rx-embedded-libgccjit rx-embedded-binutils;
        
        gcc-all = symlinkJoin {
          name = "rx-embedded-rustc_codegen_gcc-gcc";
          paths = [ rx-embedded-gcc rx-embedded-libgccjit rx-embedded-binutils ];
          preferLocalBuild = true;
        };
      in
      {
        devShell = runCommand "dummy" {
          buildInputs = [ gcc-all ];
          shellHook =
            ''
            echo "Manually update the 'gcc_path' file using the following command:"
            echo
            echo "  echo '${gcc-all}/lib' > gcc_path"
            echo
            '';
        } "";
      });
}
