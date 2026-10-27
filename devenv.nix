{ pkgs, lib, config, inputs, ... }:

{
  languages.rust = {
    enable  = true;
    channel = "stable";   # "stable" | "beta" | "nightly"

    # Uncomment for nightly with specific components
    # channel = "nightly";
    # components = [ "rustfmt" "clippy" "rust-analyzer" ];

    targets = [
      # Add cross-compilation targets as needed
      # "x86_64-unknown-linux-musl"
      # "wasm32-unknown-unknown"
    ];
  };

  packages = with pkgs; [
    # Linker & build essentials
    gcc
    pkg-config

    # Useful cargo extensions
    cargo-watch     # auto-rebuild on file changes
    cargo-expand    # macro expansion
    cargo-audit     # security advisories
    cargo-nextest   # faster test runner

    # Debugging
    lldb
  ];

  # Pre-commit hooks
  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable  = true;
  };

  scripts = {
    build.exec  = "cargo build";
    run.exec    = "cargo run";
    test.exec   = "cargo nextest run";
    watch.exec  = "cargo watch -x run";
    audit.exec  = "cargo audit";
    lint.exec   = "cargo clippy -- -D warnings";
    fmt.exec    = "cargo fmt";
  };

  env = {
    # Improves compile times with mold linker if available
    # RUSTFLAGS = "-C link-arg=-fuse-ld=mold";
    RUST_BACKTRACE = "1";
  };

  enterShell = ''
    echo "Rust $(rustc --version) environment ready"
    echo "  build → cargo build"
    echo "  run   → cargo run"
    echo "  test  → cargo nextest run"
    echo "  watch → cargo watch -x run"
  '';
}
