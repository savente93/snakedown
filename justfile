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
    fd -e toml -x taplo fmt --check {}

fix-lint:
    cargo fmt --all
    typos -w .
    fd -e toml -x taplo fmt {}
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
    mdbook build docs
    mdbook test docs

open-doc:
    cargo doc --no-deps --all-features --workspace --open
    mdbook build docs --serve

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

# Run all quality checks: fmt, lint, check, test
ci: lint test doc

# bit hacky but this should at least work across shells
# checks if there is a pr open from the current branch and if not opens one for you
# will only happen if lint and test pass and there are not uncommitted changes to tracked files
pr: ci
    gh pr list --head "$(git rev-parse --abbrev-ref HEAD)" --json author --jq ". == []" | grep -q "true"
    git diff-index --quiet HEAD --
    gh pr create --web --fill-first

zola:
    rm -rf tests/zola_test_site/content/api
    cargo r -- tests/test_pkg tests/zola_test_site api --ssg zola -e tests/test_pkg/excluded_file.py -e tests/test_pkg/excluded_module/ -vvv
    zola --root tests/zola_test_site serve
