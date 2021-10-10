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

Corpus implements a relatively simple **S**trategy for the **C**entral **O**rganization of files (usually XDG
config/data), **R**elative to some root location (usually home), where you need **U**nique **P**aths (to
correspond with usually your current directory). Basically, the folder structure for your path gets mirrored
to some alternate place.

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
               name              extension
```

Comparatively:

* `~/` -> `~/.local/share/sauce.toml`
* `~/projects/` -> `~/.local/share/sauce/projects.toml`
* `~/projects/sauce/src` -> `~/.local/share/sauce/projects/sauce/src.toml`

## Why?

Why might you want to do this, in general? For many scenarios, you would otherwise littering your folders with a
bunch of extraneous (potentially large!) files intermingled with your **actual** files.

With version control, that would mean adding a bunch of gitignore rules. There's no need if the (cache/data/config)
files aren't actually colocated with your real files.

For migrating the data from one computer to another, or auditing the set of data you have, in general: you would
otherwise have to individually trawl all folders that **might** contain relevant files. When centrally located,
you can just directly copy the root folder, and/or see the whole tree of existing data.

Practically, (I think) [sauce](https://github.com/DanCardin/sauce/blob/main/doc/comparison.md) benefits greatly
from this strategy. **If** `direnv` were to adopt this strategy, on top of the above benefits, there'd be no
security problems resulting in the need for `direnv allow`.

## CLI

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
$ corpus --ext toml --kind xdg-data -n sauce --nearest
~/.local/share/x/y.toml

$ # Get corresponding real path, given a data path
$ corpus --kind xdg-data --path ~/.local/share/x/y --source-path
~/x/y
```

### Installation

#### With Cargo

``` bash
cargo install corpus --features=binary
```

#### Download Release

- Download a pre-built binary from [Releases](https://github.com/DanCardin/corpus/releases)

### Examples

#### Central `venv`

``` bash
function venv() {
  VENV_DIR=$(corpus --kind xdg-data --name venv)
  if [ ! -d "$VENV_DIR" ]; then
    python -m venv "$VENV_DIR"
  fi
  source "$VENV_DIR/bin/activate"
}

# At ~/projects/foo
venv
# Creates ~/.local/share/venv/projects/foo

# At ~/projects/project/subproject
venv
# Creates ~/.local/share/venv/projects/project/subprocess
```

#### Central `git`

Git allows you to set two environment variables: `GIT_DIR` (the `.git` directory), and
`GIT_WORK_TREE` (the location of the root of the repo). Therefore you can (relatively simply)
adapt `git` to store all `.git/` folders by setting these variables.

``` bash
export GIT_DIR=$(corpus -n git -e git --nearest)
export GIT_WORK_TREE=$(corpus -n git -e git --nearest --source-path)
```

- for `GIT_DIR`, we want `--nearest` so that, if you `cd` into a child directory it will pick up
  the file corresponding with the closest existing git repo.
- for `GIT_WORK_TREE`, we also use `--source-path` to back track to the actual source location, given the
  data's location

(note) The above, by itself, this isn't bulletproof, since `git init` and `git clone` will exhibit some odd
behavior if you just stuck this in your bashrc/zshrc, but a little creative shell scripting (PRs welcome!) or
aliases should get the job done!

## Library

To make use of this strategy when implementing your own tools, Corpus can be used directly, as a library.

``` rust
use std::path::PathBuf;
use corpus::{builder, RootLocation};

let corpus = builder()
    .with_root("/home/.config")
    .relative_to("/home")
    .with_name("project")
    .with_extension("toml")
    .build()
    .unwrap();

let result = corpus.path("/home/foo/bar");
assert_eq!(result, PathBuf::from("/home/.config/project/foo/bar.toml"));
```

Again [Sauce](https://github.com/DanCardin/sauce) makes use of this pattern (and library) to use
this strategy for its data files!
