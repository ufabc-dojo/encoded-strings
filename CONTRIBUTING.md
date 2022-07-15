# Contributing to Encode Strings

🎉👍 First off, thanks for taking the time to contribute! 👍🎉

The following is a set of guidelines for contributing to `oxilibc`. These are
mostly guidelines, not rules. Use your best judgment and feel free to propose
changes to this document in a pull request.

## Table of contents

- [Contributing to Encode Strings](#contributing-to-oxilibc)
  - [Table of contents](#table-of-contents)
  - [Code of Conduct](#code-of-conduct)
  - [Reporting Bugs Or Suggesting Enhancements](#reporting-bugs-or-suggesting-enhancements)
  - [Pull Requests](#pull-requests)
  - [Rust Style Guide](#rust-style-guide)
  - [Useful References](#useful-references)

## Code of Conduct

The Code of Conduct for this repository [here](./CODE_OF_CONDUCT.md)

## Reporting Bugs Or Suggesting Enhancements

In the [Issues tab](TODO), click
`New issue`, write a little about the bug or feature wanted and submit it.

## Pull Requests

Always direct the Pull Request to the `dev` branch, try to be as descriptive as
possible.

Use the PR to debate, ask questions and ask for reviews (don't forget to be
respectful and follow the [Code of Conduct](./CODE_OF_CONFUCT.md)).

We follow
[Conventional Commit standard](https://www.conventionalcommits.org/en/v1.0.0/)
for this project commit messages

### Types

The types of commits that we use are:

- `feat`: A new feature
- `fix`: A bug fix
- `build`: Changes that affect the build system or external dependencies
- `chore`: Changes not related to actual code, like version bumps
- `ci`: Changes to our CI configuration files and scripts
- `docs`: Documentation only changes
- `perf`: A code change that improves performance
- `style`: Changes that do not affect the meaning of the code
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `test`: Adding missing tests or correcting existing tests

### Scope

The scope of commits that we use are:

#### Feat, Fix and Refactor

- `<name of encode-crate>`: For changes that affect that encode crate
- `<name of the module>`: For changes that affect module

#### Chore

- `version`: For changes that affect versioning
- `ide`: For changes that affect text editors and IDE configuration
- `deps`: For changes that effect dependencies version
- `<name of encode-crate>`: For changes that affect that encode crate

#### Style

- `rust`: For changes that affect Rust code style
- `rustdoc`: For changes that affect Rust-Doc documentation style

#### CI

- `build`: For CI configuration changes that affect the builder CI
- `docs`: For CI configuration changes that affect the documentation deployment
  CI
- `fmt`: For CI configuration changes that affect the code formatting check CI

## Rust Style Guide

- Don't use nightly-only features.
- The code must be formatted by `rustfmt` using `rustfmt.toml` configuration,
  otherwise, the CI will fail.
  - Install the _toolchain_ with: `rustup toolchain install nightly`, format
    your code with `cargo +nightly fmt` and be sure that
    `cargo +nightly fmt -- --check` doesn't print anything.
- Documentation should always be included when needed, for functions, methods,
  modules, etc.
- Tests, when possible, should always be included/updated with your changes.
- Always comment on what you're doing if it's not obvious, should be before the
  code that needs explaining.
- Try to be conservative about dependencies, only add if it is very necessary or
  if it will add a good amount of ergonomics with few sub-dependencies. (you can
  check the dependency tree using `cargo-tree` crate)

## Git Commit Messages

We follow and enforce
[Conventional Commit standard](https://www.conventionalcommits.org/en/v1.0.0-beta.2/)
for this project commit messages

## Useful References

- [Rust Standard Library docs](https://doc.rust-lang.org/std/)
- [API guidelines](https://rust-lang.github.io/api-guidelines/)
