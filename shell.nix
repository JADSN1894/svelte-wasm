{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell
{
  nativeBuildInputs = with pkgs.buildPackages;
    [
      just
      curl
      wget
      pkg-config
      cmake
      vim
      tmux
      rsync
      cacert
      curl
      git
      tree
      unzip
      zstd
      iproute2
      jq
      clang
      llvm
      lldb
      glibc
      rustup
      musl
      cargo-sort
      cargo-audit
      cargo-deny
      cargo-vet
      cargo-generate
      helix
      cocogitto
      difftastic
      watchexec
      hexyl
      vscodium
      nixd
      nixpkgs-fmt
      zellij
      watchexec
      nodejs_20
      wasm-pack
      fnm
    ];
  shellHook = ''
    rustup default stable
    rustup component add rust-analyzer
    rustup component add rustfmt
    rustup component add clippy
    rustup target add $(arch -m)-unknown-linux-musl 
    alias cx='cargo xtask'
    codium .
    zellij
  '';
}


