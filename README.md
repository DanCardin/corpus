# Corpus: Centrally Organized, Relative Path Uniqueness Strategy

<p align="center">
<img src="https://img.shields.io/crates/l/corpus.svg" alt="license">
<a href="https://crates.io/crates/corpus">
<img src="https://img.shields.io/crates/v/corpus.svg?colorB=319e8c" alt="Version info">
</a>
<a href="https://github.com/DanCardin/corpus/actions?query=workflow%3ATest">
<img src="https://github.com/DanCardin/corpus/workflows/Test/badge.svg" alt="Build Status">
</a> <a href="https://codecov.io/gh/DanCardin/corpus">
<img src="https://codecov.io/gh/DanCardin/corpus/branch/main/graph/badge.svg?token=U7NQIWXWKW"/>
</a><br>
</p>

## Introduction

Corpus implements a relatively simple Strategy for the Central Organization of files (usually XDG
config/data), relative to some root location (usually home), where you need Unique paths (to
correspond with usually your current directory).

The purpose is probably most easily explained using the motivating example project:
[Sauce](https://github.com/DanCardin/sauce). Sauce (similar to `direnv`) records directory-specific
environment variables (among other things) for you to be able to activate once you're in that
directory.

**Unlike** `direnv`, the files containing the this data are not in a local `.envrc` file, but rather
organized centrally. For example, if you are at `~/projects/sauce`, the corresponding data path
would be:

``` text
~/.local/share/sauce/projects/sauce.toml
|             |     |             |     |
+------+------+--+--+------+------+--+--+
       |         |         |         |
  XDG_DATA_DIR   |    relative path  |
              project             extension
```

Comparatively, `~/projects/` would yield `~/.local/share/sauce/projects.toml`, and
`~/projects/sauce/src` would yield `~/.local/share/sauce/projects/sauce/src.toml`. Essentially the
idea is to replicate the same local folder structure...but elsewhere!

## Why?

Why might you want to do this? For many scenarios, you would otherwise littering your folders with a
bunch of extraneous (potentially large!) files intermingled with your **actual** files.

For `sauce`, copying the data from one computer to another is as simple as copying the root
location: `~/.local/share/sauce`. For `direnv` (if you configured it as such), it'd mean copying
`~/.local/share/direnv`. In my book, that beats combing through all potential locations you might
have placed some config/secrets.

It also means you dont have to pollute your version control (if you're using one) ignore
configuration with using to avoid committing data/secrets specific to you.

## How

### (CLI) Installation

``` bash
#### With Cargo

- `cargo install corpus --features=binary`

#### Download Release

- Download a pre-built binary from [Releases](https://github.com/DanCardin/corpus/releases)
```

The `corpus` CLI command can be used to interactively determine paths. This can commonly be used to
adapt (appropriately configurable) tools to use this strategy themselves!

``` bash
$ # Get the "corpus" path for the current directory
$ corpus --ext toml --kind xdg-data --name sauce
~/.local/share/x/y/z.toml

$ # Get the "corpus" path for a specific directory
$ corpus --ext toml --kind xdg-data --path <path> -n sauce
~/.local/share/<path>.toml

$ # Get the nearest ancestor directory that actually exists
$ corpus --ext toml --kind xdg-data --nearest -n sauce
~/.local/share/x/y.toml

$ # Get corresponding real path, given a data path
$ corpus --kind xdg-data --source-path --path ~/.local/share/x/y
~/x/y
```

### For example, central `git`

Git allows you to set two environment variables: `GIT_DIR` (the `.git` directory), and
`GIT_WORK_TREE` (the location of the root of the repo).

Therefore you can (relatively simply) adapt `git` to store all `.git/` folders centrally with a
little creative bash.

``` bash
export GIT_DIR=$(corpus --nearest -n git -e git)
export GIT_WORK_TREE=$(corpus --nearest --source-path -n git -e git)
```

- for `GIT_DIR`, we want `--nearest` so that, if you `cd` into a child directory it will pick up the
  file corresponding with the closest existing git repo.
- for `GIT_WORK_TREE`, we also use `--source-path` to back-trace the repo root location, given the
  data's location

By itself this isn't bulletproof, since `git init` and `git clone` will exhibit some odd behavior if
you just stuck this in your bashrc/zshrc, but a little creative shell scripting or aliases should
get the job done!

## Library

It can also be used as a library, to make use of this strategy when implementing your own tools.

``` rust
use std::path::PathBuf;
use corpus::{builder, RootLocation};

let corpus = builder()
    .with_root("/home/.config")
    .relative_to("/home")
    .at_path("/home/foo/bar")
    .with_name("project")
    .with_extension("toml")
    .build()
    .unwrap();

let result = corpus.path();
assert_eq!(result, PathBuf::from("/home/.config/project/foo/bar.toml"));
```
