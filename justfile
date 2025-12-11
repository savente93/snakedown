#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

log := "warn"

alias b := build
alias t := test
alias l := lint
alias fl := fix-lint

export JUST_LOG := log

lint:
    cargo clippy --all --all-targets --all-features -- --deny warnings
    cargo fmt --all -- --check
    typos .
    taplo fmt --check .

fix-lint:
    cargo fmt --all
    typos -w .
    taplo fmt --check .
    cargo clippy --fix


# Run tests
test:
    cargo test --all

# Build the project
build:
    cargo build

# Build the project
build-release:
    cargo build --release

doc:
    cargo doc --no-deps --all-features --workspace

open-doc:
    cargo doc --no-deps --all-features --workspace --open

cov:
    cargo llvm-cov --locked --all-features  --open

# Clean the target directory
clean:
    cargo clean

newest:
    cargo upgrade --incompatible --recursive
    cargo +nightly update --breaking -Z unstable-options

semver:
    cargo semver-checks

install-dev-tools:
    cargo install cargo-binstall
    cargo binstall cargo-semver -y
    cargo binstall cargo-edit -y
    cargo binstall git-cliff -y
    cargo binstall typos-cli -y
    cargo binstall taplo-cli -y
    cargo binstall bacon -y

# Run all quality checks: fmt, lint, check, test
ci: lint test

# bit hacky but this should at least work across shells
# checks if there is a pr open from the current branch and if not opens one for you
# will only happen if lint and test pass and there are not uncommitted changes to tracked files
pr: ci
    gh pr list --head "$(git rev-parse --abbrev-ref HEAD)" --json author --jq ". == []" | grep -q "true"
    git diff-index --quiet HEAD --
    gh pr create --web --fill-first

zola:
    cargo r -- tests/test_pkg tests/zola_test_site/content/ --ssg zola -e tests/test_pkg/excluded_file.py -e tests/test_pkg/excluded_module/
    zola --root tests/zola_test_site build
