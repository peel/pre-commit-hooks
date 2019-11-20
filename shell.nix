with import <nixpkgs> {};

mkShell {
  buildInputs = with pkgs; [ cargo rustc pre-commit darwin.apple_sdk.frameworks.Security ];
  shellHook = ''
   pre-commit install
   pre-commit run --all-files
  '';
}
