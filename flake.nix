{
  description = "LEZ Wallet development environment";

  inputs.nixpkgs.url = "https://releases.nixos.org/nixos/26.05/nixos-26.05.9729.c3eea5b2156d/nixexprs.tar.xz";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      eachSystem = nixpkgs.lib.genAttrs systems;
    in {
      devShells = eachSystem (system:
        let pkgs = import nixpkgs { inherit system; };
        in {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustup
              pkg-config openssl clang llvmPackages.libclang cmake protobuf pcsclite
            ];
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib";
            RUSTUP_TOOLCHAIN = "1.98.1";
            RISC0_DEV_MODE = "1";
            shellHook = ''
              rustup toolchain install 1.98.1 --component clippy --component rustfmt
              export PATH="$(dirname "$(rustup which rustc)"):$PATH"
            '';
          };
        });
    };
}
