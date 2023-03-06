# erdtree (et)

[![Build status](https://github.com/solidiquis/erdtree/actions/workflows/ci.yml/badge.svg)](https://github.com/solidiquis/erdtree/actions)
[![Crates.io](https://img.shields.io/crates/v/erdtree.svg)](https://crates.io/crates/erdtree)

A modern, vibrant, and multi-threaded file-tree visualizer and disk usage analyzer that respects hidden files and `.gitignore` rules - basically if [tree](https://en.wikipedia.org/wiki/Tree_(command)) and [du](https://en.wikipedia.org/wiki/Du_(Unix)) had a baby.

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/demo.png?raw=true" alt="failed to load picture" />
</p>

## Table of Contents

* [Description](#description)
* [Usage](#usage)
* [Installation](#installation)
* [Disambiguations](#disambiguations)
  - [Disk usage](#disk-usage)
  - [File without read permissions](#files-without-read-permissions)
  - [File coloring](#file-coloring)
  - [tree command](#tree-command)
  - [Symlinks](#symlinks)
  - [Advantages over exa --tree](#advantages-over-exa---tree)
  - [Icons](#icons)
* [Rules for Contributing and Feature Requests](#rules-for-contributing-and-feature-requests)
* [Special Thanks](#special-thanks)
* [Questions you might have](#questions-you-might-have)

## Description

**erdtree** is a modern alternative to `tree` and `du` in that it:
- offers a minimal and user-friendly CLI
- respects hidden files and `.gitignore` rules by default
- displays file sizes in human-readable format by default
- traverses directories in a parallel manner (4 threads by default)
- displays files using ANSI colors by default
- supports icons! (checkout the [Icons](#icons) section before using)

## Usage
```
$ et -h
erdtree (et) is a multi-threaded filetree visualizer and disk usage analyzer.

Usage: et [OPTIONS] [DIR]

Arguments:
  [DIR]  Root directory to traverse; defaults to current working directory

Options:
  -g, --glob <GLOB>            Include or exclude files using glob patterns
      --iglob <IGLOB>          Include or exclude files using glob patterns; case insensitive
      --glob-case-insensitive  Process all glob patterns case insensitively
  -H, --hidden                 Show hidden files; disabled by default
      --ignore-git             Disable traversal of .git directory when traversing hidden files; disabled by default
  -I, --icons                  Display file icons; disabled by default
  -i, --ignore-git-ignore      Ignore .gitignore; disabled by default
  -l, --level <NUM>            Maximum depth to display
  -s, --sort <SORT>            Sort-order to display directory content [default: none] [possible values: name, size, none]
  -S, --follow-links           Traverse symlink directories and consider their disk usage; disabled by default
  -t, --threads <THREADS>      Number of threads to use [default: 4]
  -h, --help                   Print help (see more with '--help')
  -V, --version                Print version
```

## Installation

### crate.io

1. Make sure you have [Rust and its toolchain](https://www.rust-lang.org/tools/install) installed.
2. `$ cargo install erdtree`

### Homebrew

```
$ brew install erdtree
```

### Releases
Binaries for common architectures can be downloaded from latest releases.

Other means of installation to come.

## Disambiguations

### Disk usage

As recommended in [IEC 80000-13](https://en.wikipedia.org/wiki/ISO/IEC_80000#cite_note-80000-13:2008-14), this command will report sizes
using SI units rather than binary units. As such you can expect `1KB = 1000B` and not `1KiB = 1024B`. 

Additionally:
- A directory will have a size equal to the sum of the sizes of all of its entries. The size of the directory itself is negligble and isn't taken into account.
- Hidden files, files excluded by `.gitignore`, and files excluded via globbing will be ommitted from the total memory size of their parent directories.
- Special files such a named pipes, sockets, etc. have negligible sizes so their disk usage aren't reported.

### Files without read permissions

Files that don't have read permissions will appear but won't have their disk sizes reported. If they are directories they will not be traversed. Additionally, their size will not be included in their parent directory's total.

### File coloring

Files are printed in ANSI colors specified according to the `LS_COLORS` environment variable on GNU/Linux systems. In its absence [a default value](https://docs.rs/lscolors/latest/src/lscolors/lib.rs.html#221) is used.

**Note for MacOS**: MacOS uses the `LSCOLORS` environment variable to determine file colors for the `ls` command which is formatted very differently from `LS_COLORS`. MacOS systems will fall back on the aforementioned default value unless the user defines their own `LS_COLORS` environment variable.

### `tree` command

This is not a rewrite of the `tree` command thus it should not be considered a 1-to-1 port.

### Symlinks

Symlinks are not followed by default. `-S, --follow-links` enables symlink following and reports their disk usages. Descendents of directories that are targets of a symlink will also have branches of a different color.

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/symlink.png?raw=true" alt="failed to load png" />
</p>

### Advantages over `exa --tree`

[Exa](https://github.com/ogham/exa) is a powerful modern equivalent of the `ls` command which gives the option to print a tree-view of a specified directory, however the primary differences between `exa --tree` and `et` are:
- `exa --tree --git-ignore` doesn't respect `.gitignore` rules on a per directory basis whereas `et` does. With `exa` the root's `.gitignore` is considered, but if child directories have their own `.gitignore` they are disregarded and all of their contents will be printed.
- `et` displays the total size of a directory as the sum of all of its entries' sizes whereas `exa` [does not support this](https://github.com/ogham/exa/issues/91). This makes sorting directories in the tree-view by size dubious and unclear. Below are screenshots comparing equivalent usages of `et` and `exa`, using long option names for clarity.

#### exa
<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/exa.png?raw=true" alt="failed to load png" />
</p>

#### erdtree
<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/et.png?raw=true" alt="failed to load png" />
</p>

### Icons

For icons to render properly it is required that the font you have hooked up to your terminal emulator contain the glyphs that `erdtree` expects in order to properly render icons which is why `-I, --icons` is an opt-in feature.

If your icons look something like this:

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/broken_icons.png?raw=true" alt="failed to load png" />
</p>

this means that the font you are using doesn't include the relevant glyphs. To resolve this issue download a [NerdFont](https://www.nerdfonts.com/) and hook it up to your terminal emulator.

## Rules for Contributing and Feature Requests

Happy to accept contributions but please keep the following in mind:
- If you're doing some minor refactoring and/or code cleanup feel free to just submit a PR.
- If you'd like to add a feature please open up an issue and receive approval first.
- Feature adds generally require tests.

Feature requests in the form of issues in general are welcome.

## Special thanks

- to Reddit user `/u/johnm` for suggesting that different SI prefixes be colored differently for better visual feedback.
- to Reddit user `/u/lucca_huguet` (can be found as [luccahuguet](https://github.com/luccahuguet) on Github) for suggesting that the compiled `erdtree` binary be shorted to `et`.
- to all contributors :]

## Questions you might have

_Q: Why did you make this? It's totally unnecessary._

A: Ennui.

_Q: Why is it called erdtree?_

A: It's a reference to Elden Ring.

_Q: Is it any good?_

A: Yes.

_Q: Is it blazingly fast?_

A: Should be. I wrote it in Rust.

<img src="https://i.redd.it/t7ns9qtb5gh81.jpg">
