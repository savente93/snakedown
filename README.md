# SnakeDown 🐍🕵️‍♂️

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![codecov](https://codecov.io/gh/savente93/SnakeDown/branch/main/graph/badge.svg)](https://codecov.io/gh/savente93/SnakeDown)


A Python API extractor written in Rust. I got frustrated with the standard tools used for documentation generation used in Python, namely that all of the ones I've worked with have at least one of the following to characteristics:

1. slow
2. poor developer experience (e.g. [Sphinx](https://www.sphinx-doc.org/en/master/) lacks a `serve` functionality)

Number 1 is particularly a concern for packages that are slow to import, or have a convoluted structure (which is the case for some packages I work with)

There are many excellent Static Site Generators like [Hugo](https://gohugo.io) or [Zola](https://www.getzola.org) that are fast and offer a much better experience when developing. Typically, documentation are static pages so they should be a perfect fit, except for one issue: automatic API reference generation. This was a big showstopper... until now!

SnakeDown is an experiment to get to a place where using Hugo or Zola for python documentation generation is a viable option.


(Currently mostly for shits and giggles, but who knows.)

## Design goals:

While the project might not cover all of these goals yet, these are some of the things I try to keep in mind when working on SnakeDown (in loose order of precedence):

1. **Sensible defaults over infinite configurability**. Infinite configuration comes with a lot of complexity both for user and developer, and in my opinion it is not worth it, especially at the stage SnakeDown is at now. I'd rather do 10 things right 80% of the time, then 2 things right 99.999% of the time.
2. **Correctness over speed** While speed is an explicit goal of this project, correctness should take precedent. Doing the wrong thing extremely fast is useless.
3. **Be fast** We only *parse* the source code to extract the docstrings from it, but we don't actually have to do any execution. While this might mean we can't always fully expand everything, type signatures are constants or names in the vast majority of cases, so this is a decent alternative. This by itself should make us Fast Enough :tm:
4. **Be fun** This is a personal project for the time being, and I like to have fun with those.

## Status

Currently, SnakeDown is solidly in the MVP state. While I am quite happy with it so far, you use it at your own risk. That said, I always welcome bug reports and feature requests if you do decide to try it! Below is a loose planning of the features I want to work on


## Loose Roadmap:

- [x] Walk file tree to find python objects
- [x] Parse them to extract documentation
- [x] Dump documentation in similar file structure to original package
- [x] Fill out the CLI
- [ ] Test output in SSGs (zola, hugo for now, please submit a feature request if you want others included)
- [x] Logging at appropriate levels
- [ ] Parse/render docstring formats like numpy and google so we can render them better
- [ ] Configuration file
- [ ] Support multiple formats? (md, rst)
- [ ] Do reference linking inside the docs
- [ ] Do reference linking to external docs
- [ ] Benchmarking & optimisation

## FAQ

### Is SnakeDown dead?

No. It is a personal project for the time being so while there may be persiods of inactivity, I intend to keep working on it until it serves my personal needs. (Poe's law dictates that I mention this is mostly a joke. It's a common question for FOSS projects that don't have the high velocity of larger projects and this is a tongue in cheek way of beating this question to the punch)

### When is the next release?

To quote one of the giants in our field BurtnSushi:

> ripgrep is a project whose contributors are volunteers. A release schedule adds undue stress to said volunteers. Therefore, releases are made on a best effort basis and no dates will ever be given.

> An exception to this can be high impact bugs. If a ripgrep release contains a significant regression, then there will generally be a strong push to get a patch release out with a fix. However, no promises are made.

(source: [ripgrep](https://github.com/BurntSushi/ripgrep/blob/94305125ef33b86151b6cd2ce2b33d641f6b6ac3/FAQ.md#release))


The same applies to SnakeDown.

### Why is it called SnakeDown?

Three main reasons:

1. I was first going to call it "SnakeOil" for extraing things from snakes of dubious value but that name was already taken
2. it's about shaking down your python code for docs
3. I find it funny

### Did people actually ask these questions?

Not yet, but I suspect they will.


## Dev tools
To develop SnakeDown you'll want to have these tools installed:

- [`just`](https://github.com/casey/just) A command runner to run (and document) workflows we run, including installing dev and publish dependencies
- [`typos-cli`](https://github.com/crate-ci/typos) Fixing typos... not that we make any... but you know, just in case.
- [`taplo-cli`](https://github.com/tamasfe/taplo) Keeping our `.toml` files nice and clean
- [`bacon`](https://github.com/Canop/bacon) A runner that will watch your files and run checks, tests, linting etc. when they change. Very useful while developing

##  Publishing Tools
If you have to publish, or otherwise fiddle with dependencies of SnakeDown you'll want these installed as well:
- [`cargo-semver`](https://github.com/obi1kenobi/cargo-semver-checks) A cargo plugin to check that we haven't accidentally broken our API when we didn't mean to.
- [`cargo-edit`](https://github.com/killercup/cargo-edit) A cargo plugin for managing dependencies, incl updating them.
- [`git-cliff`](https://github.com/orhun/git-cliff) A neat tool to generate our changelog

## Template

This repo was initially setup using [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) and [this template](https://github.com/savente93/rust-template)
