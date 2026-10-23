{
  inputs.rust-overlay = {
    url = "git+https://github.com/oxalica/rust-overlay";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    { nixpkgs, rust-overlay, ... }:
    {
      devShells.x86_64-linux.default = nixpkgs.legacyPackages.x86_64-linux.mkShell {
        packages = [ rust-overlay.packages.x86_64-linux.rust_1_89_0 ];
      };
    };
}
