# erdtree (et)

[![Build status](https://github.com/solidiquis/erdtree/actions/workflows/ci.yml/badge.svg)](https://github.com/solidiquis/erdtree/actions)
[![Crates.io](https://img.shields.io/crates/v/erdtree.svg)](https://crates.io/crates/erdtree)

A modern, vibrant, and multi-threaded file-tree visualizer and disk usage analyzer that respects hidden files and `.gitignore` rules by default - basically if [tree](https://en.wikipedia.org/wiki/Tree_(command)) and [du](https://en.wikipedia.org/wiki/Du_(Unix)) had a baby.

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/demo.png?raw=true" alt="failed to load picture" />
</p>

## Table of Contents

* [Description](#description)
* [Usage](#usage)
* [Installation](#installation)
* [Info](#info)
  - [Configuration file](#configuration-file)
  - [Binary prefix or SI prefix](#binary-prefix-or-si-prefix)
  - [Logical or physical disk usage](#logical-or-physical-disk-usage)
  - [How are directory sizes computed](#how-are-directory-sizes-computed)
  - [Symlinks](#symlinks)
  - [Hardlinks](#hardlinks)
  - [File coloring](#file-coloring)
  - [Icons](#icons)
* [Comparisons against similar programs](#comparisons-against-similar-programs)
  - [tree command](#tree-command)
  - [Advantages over exa --tree](#advantages-over-exa---tree)
  - [dua](#dua)
  - [dust](#dust)
* [Rules for Contributing and Feature Requests](#rules-for-contributing-and-feature-requests)
* [Special Thanks](#special-thanks)
* [Questions you might have](#questions-you-might-have)

## Description

**erdtree** is a modern alternative to `tree` and `du` in that it:
- offers a minimal and user-friendly CLI
- respects hidden files and `.gitignore` rules by default
- displays file sizes in human-readable format by default
- leverages parallism to traverse the file-system
- displays files using ANSI colors by default
- supports icons! (checkout the [Icons](#icons) section before using)

If the chosen defaults don't meet your requirements and you don't want to bloat your shell configs with aliases, you can use a [configuration file](#configuration-file) instead.

## Usage
```
erdtree (et) is a multi-threaded filetree visualizer and disk usage analyzer.

Usage: et [OPTIONS] [DIR]

Arguments:
  [DIR]  Root directory to traverse; defaults to current working directory

Options:
  -d, --disk-usage <DISK_USAGE>  Print physical or logical file size [default: logical] [possible values: logical, physical]
  -g, --glob <GLOB>              Include or exclude files using glob patterns
      --iglob <IGLOB>            Include or exclude files using glob patterns; case insensitive
      --glob-case-insensitive    Process all glob patterns case insensitively
  -H, --hidden                   Show hidden files; disabled by default
      --ignore-git               Disable traversal of .git directory when traversing hidden files; disabled by default
  -I, --icons                    Display file icons; disabled by default
  -i, --ignore-git-ignore        Ignore .gitignore; disabled by default
  -l, --level <NUM>              Maximum depth to display
  -n, --scale <NUM>              Total number of digits after the decimal to display for disk usage [default: 2]
  -s, --sort <SORT>              Sort-order to display directory content [possible values: name, size, size-rev]
      --dirs-first               Always sorts directories above files
  -S, --follow-links             Traverse symlink directories and consider their disk usage; disabled by default
  -t, --threads <THREADS>        Number of threads to use [default: 4]
      --suppress-size            Omit disk usage from output; disabled by default
      --no-config                Don't read configuration file
  -h, --help                     Print help (see more with '--help')
  -V, --version                  Print version
```

## Installation

### crate.io

1. Make sure you have [Rust and its toolchain](https://www.rust-lang.org/tools/install) installed.
2. `$ cargo install erdtree`

### Homebrew-core

```
$ brew install erdtree
```

### Releases
Binaries for common architectures can be downloaded from latest releases.

Other means of installation to come.

## Info

### Configuration file

If `erdtree`'s out-of-the-box defaults don't meet your specific requirements, you can set your own defaults using a configuration file.

To create an `erdtree` configuration file you can either:
- Create a file located at `ERDTREE_CONFIG_PATH` which you set, or
- Create `$HOME/.erdtreerc`

The format of a config file is as follows:
- Every line is an `erdtree` option/argument.
- Lines starting with `#` are considered comments and are thus ignored.

Arguments passed to `erdtree` take precedence. If you have a config that you would like to ignore without deleting you can use `--no-config`.

Here is an example of a valid config:

```
$ cat $HOME/.erdtreerc
--level 2
--icons
--scale 3

# You can use the short names too
-s size
```


### Binary prefix or SI Prefix

Disk usage is reported using binary prefixes, thus you can expect `1 kebibyte` to equal `1024 bytes` (i.e `1 KiB = 1024 B`).

### Logical or physical disk usage

Logical sizes are reported by default but you can toggle the reporting to physical sizes which takes into account compression, sparse files, and actual blocks allocated to a particular file via the following option:

```
-d, --disk-usage <DISK_USAGE>  Print physical or logical file size [default: logical] [possible values: logical, physical]
```

### How are directory sizes computed

- A directory will have a size equal to the sum of the sizes of all of its entries.
- Hidden files, files excluded by `.gitignore`, and files excluded via globbing will be ommitted from the disk usages of their parent directories.
- Files/Directories that don't have read permissions will be ommitted from the disk usages of their parent directories.
- Special files such a named pipes, sockets, etc. have negligible sizes so their disk usage aren't reported.

### Symlinks

- If symlink following is not enabled via `-S, --follow-links`, the disk usages of their target will not be reported nor considered.
- If symlink following is enabled the size of the target will be reported and considered as part of the total of the symlink's ancestral directories.
- The parts of the file-tree that branch from the symlink that's followed are printed in a different color.

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/symlink.png?raw=true" alt="failed to load png" />
</p>

### Hardlinks

If you happen to have multiple hardlinks pointing to the same underlying inode in a given file-tree, everything subsequent to the first will be skipped and ignored as to not be double counted in the overall disk-usage.

### File coloring

Files are printed in ANSI colors specified according to the `LS_COLORS` environment variable on GNU/Linux systems. In its absence [a default value](https://docs.rs/lscolors/latest/src/lscolors/lib.rs.html#221) is used.

**Note for MacOS**: MacOS uses the `LSCOLORS` environment variable to determine file colors for the `ls` command which is formatted very differently from `LS_COLORS`. MacOS systems will fall back on the aforementioned default value unless the user defines their own `LS_COLORS` environment variable.

### Icons

Icons (enabled with `I, --icons`) are an opt-in feature because for icons to render properly it is required that the font you have hooked up to your terminal emulator contains the glyphs necessary to properly render icons.

If your icons look something like this:

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/broken_icons.png?raw=true" alt="failed to load png" />
</p>

this means that the font you are using doesn't include the relevant glyphs. To resolve this issue download a [NerdFont](https://www.nerdfonts.com/) and hook it up to your terminal emulator.

## Comparisons against similar programs

### `tree` command

This is not a rewrite of the `tree` command thus it should not be considered a 1-to-1 port. While the spirit of `tree` is maintained `erdtree` there are more differences than there are similarities.

### Advantages over `exa --tree`

[Exa](https://github.com/ogham/exa) is a powerful modern equivalent of the `ls` command which gives the option to print a tree-view of a specified directory, however the primary differences between `exa --tree` and `et` are:
- `exa --tree --git-ignore` doesn't respect `.gitignore` rules on a per directory basis whereas `et` does. With `exa` the root's `.gitignore` is considered, but if child directories have their own `.gitignore` they are disregarded and all of their contents will be printed.
- `et` displays the total size of a directory as the sum of all of its entries' sizes whereas `exa` [does not support this](https://github.com/ogham/exa/issues/91). This makes sorting directories in the tree-view by size dubious and unclear. Below are screenshots comparing equivalent usages of `et` and `exa`, using long option names for clarity.

#### exa
<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/exa_cmp.png?raw=true" alt="failed to load png" />
</p>

#### erdtree
<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/et_cmp.png?raw=true" alt="failed to load png" />
</p>

### dua

[dua](https://github.com/Byron/dua-cli) is a fantastic interactive disk usage analyzer that serves as a modern alternative to [ncdu](https://en.wikipedia.org/wiki/Ncdu). If you're in the mood for something interactive, `dua` might suit you more. If you'd rather do a quick analysis of your file-tree and disk-usage without spinning up an entire terminal UI then go with `erdtree`.

### dust

[dust](https://github.com/bootandy/dust) is another fantastic tool that heavily overlaps with `erdtree` in functionality. The biggest differences between the two:
- `erdtree` defaults to respecting hidden file and `.gitignore` rules.
- `dust` only shows a limited set of files in the file-tree by default, but also defaults to showing the largest files first.
- `erdtree` keeps the spirit of `tree` with respect to the output while `dust` has a horizontal bar graph.

I haven't used `dust` substantially so these are the immediate differences I came across. Try `dust` and `erdtree` and see which best suits you.

## Rules for Contributing and Feature Requests

Happy to accept contributions but please keep the following in mind:
- If you're doing some minor refactoring and/or code cleanup feel free to just submit a PR.
- If you'd like to add a feature please open up an issue and receive approval first unless you've previously contributed. You can also start a [discussion](https://github.com/solidiquis/erdtree/discussions/49).
- If new arguments/options are added please do your best to keep them sensibly alphabetized.
- The code is heavily documented so please follow suit. `cargo doc --open` can be extremely helpful.
- Feature adds generally require tests.

Feature requests in the form of issues in general are welcome.

## Special thanks

- to Reddit user `/u/johnm` for suggesting that different binary prefixes be colored differently for better visual feedback.
- to Reddit user `/u/Freeky` for suggestions on how to handle hardlinks and physical disk sizes.
- to Reddit user `/u/lucca_huguet` (can be found as [luccahuguet](https://github.com/luccahuguet) on Github) for suggesting that the compiled `erdtree` binary be shorted to `et`.
- to all of the lovely people on [this](https://www.reddit.com/r/rust/comments/11ioq1n/erdtree_v120_a_modern_multithreaded_alternative/) Reddit thread who helped shaped `erdtree` with their very valuable feedback.
- to [messense](https://github.com/messense) for getting this on Homebrew-core!
- to all contributors :]

## Questions you might have

_Q: Why did you make this? It's totally unnecessary._

A: Ennui.

_Q: Is it any good?

A: Don't ask me, ask this dude/dudette:

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/tarnished_i.png?raw=true" alt="failed to load png" />
</p>

and this dude/dudette:

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/tarnished_ii.png?raw=true" alt="failed to load png" />
</p>

_Q: Why is it called erdtree?_

A: It's a reference to Elden Ring.

_Q: Is it any good?_

A: Yes.

_Q: Is it blazingly fast?_

A: Should be. I wrote it in Rust.

<img src="https://i.redd.it/t7ns9qtb5gh81.jpg">
