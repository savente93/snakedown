# Contributing to snakedown

Thanks for wanting to contribute! There are many ways to contribute and we
appreciate any level you're willing to do. This repository is just for snakedown itself
if the issue, request or contribution is to do with the theme, please report that in the
specific repository.

## Feature Requests

Need some new functionality? You can let us know by opening an
[issue][new issue]. Please include a good description of what you would like
and what you are hoping to accomplish with it. If you have considered alternatives
or current work arounds it is often also helpful to describe those.

In general if your feature request is well written and complete it vastly increaces the chances it will be picked up soon. In general we don't ask for information for no reason, so if you omit things that may slow things down.

## Bug Reports

Please let us know about what problems you run into, whether in behavior or
ergonomics of API. You can do this by opening an [issue][new issue].

When you do so please include the version you were using as well good descriptions
of what you were doing that may have triggered the error and what
you expect to happen instead (unless it is trivial for example when you are reporting a crash)

In general if your bug report is well written and complete it vastly increaces the chances it will be picked up soon. In general we don't ask for information for no reason, so if you omit things that may slow things down.


## Pull Requests

Looking for an idea? Check our [issues][issues]. If the issue looks open ended,
it is probably best to post on the issue how you are thinking of resolving the
issue so you can get feedback early in the process. We want you to be
successful and it can be discouraging to find out a lot of re-work is needed.
We encourage you to reach out and ask questions early in the process if you are uncertain.

Don't know where to start? check out the issues labeled [good first issue][good first issue] and [mentoring available][mentoring available]. In general these are beginner friendly with more detailed instructions.

Already have an idea?  It might be good to first [create an issue][new issue]
to propose it so we can make sure we are aligned and lower the risk of having
to re-work some of it and the discouragement that goes along with that.

### Process

As a heads up, we'll be running your PR through the following gauntlet:
- warnings turned to compile errors
- `cargo test`
- `rustfmt`
- `clippy`
- `rustdoc`
- `taplo` (toml formatter)
- `codecov`
- [`committed`](https://github.com/crate-ci/committed) as we use [Conventional](https://www.conventionalcommits.org) commit style. Ideally the commit message shouldn't just say what was done but also why.
- [`typos`](https://github.com/crate-ci/typos) to check spelling

In generally you can make sure these are okay by installing the `pre-commit` hooks in this repository. Not everything can be checked automatically though.

We also don't allow "TODO" comments in the code unless they also link to an issue, since TODO comments usually get forgotten and overlooked.

We request that the commit history gets cleaned up so that that commits are atomic, meaning they are complete and have a single responsibility. A complete commit should build, pass tests, update documentation and tests, and not have dead or commented out code.

PRs should tell a cohesive story, with refactor and test commits that keep the
fix or feature commits simple and clear.


Specifically, we would encourage
- File renames be isolated into their own commit
- Add tests in a commit before their feature or fix, showing the current behavior (i.e. they should pass).
  The diff for the feature/fix commit will then show how the behavior changed,
  making the commit's intent clearer to reviewers and the community, and showing people that the
  test is verifying the expected state.
  - e.g. [clap#5520](https://github.com/clap-rs/clap/pull/5520)

We understand having a clean history requires more advanced git skills;
feel free to ask us for help!

We might even suggest where it would work to be lax.
We also understand that editing some early commits may cause a lot of churn
with merge conflicts which can make it not worth editing all of the history.

#### Coverage
Coverage in Rust can be a bit fineky at times, and additionally coverage doesn't always tell the whole story, so we usually don't enforce hard limits on coverage. That being said, we do like to keep our coverage high, so if you don't cover something, please have good explanation as to why!

#### Organisation

For code organization, we recommend
- Grouping `impl` blocks next to their type (or trait)
- Grouping private items after the `pub` item that uses them.
  - The intent is to help people quickly find the "relevant" details, allowing them to "dig deeper" as needed.  Or put another way, the `pub` items serve as a table-of-contents.
  - The exact order is fuzzy; do what makes sense

### Dev tips

You are not required to have the following tools installed to work on snakedown, but they can make things a lot easier:

- [`just`](https://github.com/casey/just) A command runner to run (and document) workflows we run, including installing dev and publish dependencies.
- [`pre-commit`](https://pre-commit.com) This will run lints when you try to commit so you don't fail CI tasks unnecessarily.
- [`bacon`](https://github.com/Canop/bacon) A runner that will watch your files and run checks, tests, linting etc. when they change. Very useful while developing for fast feedback cycles.
- [`gh`](https://github.com/cli/cli) Can be used this to quickly open PRs when done working locally and make sure they aren't duplicated. Quite convenient
- [`zola`](https://github.com/getzola/zola) not technically required as our CI will test things with it, but very handy to have locally to iterate faster.

Also keep in mind not all our rules have to be met at every single stage. It is totally allowed to iterate/prototype until you are happy with things, and then clean up after!

##  Publishing

For this project we have a [`release-plz`](https://release-plz.dev/docs) action setup. This gets updated automatically, and to release to all the places we distribute too all you have to do is edit and merge that release PR.

[issues]: https://github.com/savente93/snakedown/issues
[new issue]: https://github.com/savente93/snakedown/issues/new
[good first issue]: https://github.com/savente93/snakedown/issues?q=state%3Aopen%20label%3A%22good%20first%20issue%22
[mentoring available]: https://github.com/savente93/snakedown/issues?q=state%3Aopen%20label%3Amentoring-available

## Acknolwegements
- thank you to [Ed Page](https://github.com/epage). These guidelines were adapted from [this](https://github.com/epage/_rust) template which he wrote.
