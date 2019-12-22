with import <nixpkgs> {};

# nix-build -A yaml bootstrap.nix

{
  yaml = idrisPackages.callPackage ./deps.nix {};
}