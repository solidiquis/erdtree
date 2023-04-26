# erdtree (erd)

[![Build status](https://github.com/solidiquis/erdtree/actions/workflows/ci.yml/badge.svg)](https://github.com/solidiquis/erdtree/actions)
[![Crates.io](https://img.shields.io/crates/v/erdtree.svg)](https://crates.io/crates/erdtree)
[![Packaging status](https://repology.org/badge/tiny-repos/erdtree.svg)](https://repology.org/project/erdtree/versions)
[![Crates.io](https://img.shields.io/crates/d/erdtree)](https://crates.io/crates/erdtree)

*Erdtree* is a modern, cross-platform, and multi-threaded filesystem and disk-usage analysis tool. The following are some feature highlights:
* Respects hidden file and gitignore rules by default.
* Supports regular expressions and glob based searching by file-type.
* Supports Unix-based file permissions (Unix systems only).
* Comes with a variety of layouts.
* Support icons.
* Colorized with `LS_COLORS`.

You can think of `erdtree` as a combination of `du`, `tree`, `find`, and `ls`.

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/showcase_top.png?raw=true" alt="failed to load picture" />
</p>

## Table of Contents

* [Usage](#usage)
* [Installation](#installation)
* [Documentation](#documentation)
  - [Configuration file](#configuration-file)
  - [Parallelism](#parallelism)
  - [Binary prefix or SI prefix](#binary-prefix-or-si-prefix)
  - [Logical or physical disk usage](#logical-or-physical-disk-usage)
  - [How are directory sizes computed](#how-are-directory-sizes-computed)
  - [Symlinks](#symlinks)
  - [Hardlinks](#hardlinks)
  - [File coloring](#file-coloring)
  - [Icons](#icons)
  - [Completions](#completions)
  - [Plain view](#plain-view)
* [Comparisons against similar programs](#comparisons-against-similar-programs)
  - [tree command](#tree-command)
  - [Advantages over exa --tree](#advantages-over-exa---tree)
  - [dua](#dua)
  - [dust](#dust)
* [Rules for Contributing and Feature Requests](#rules-for-contributing-and-feature-requests)
* [Special Thanks](#special-thanks)
* [Questions you might have](#questions-you-might-have)

## Usage

```
erdtree (erd) is a cross-platform multi-threaded filesystem and disk usage analysis tool.

Usage: erd [OPTIONS] [DIR]

Arguments:
  [DIR]  Directory to traverse; defaults to current working directory

Options:
  -C, --force-color                Turn on colorization always
  -d, --disk-usage <DISK_USAGE>    Print physical or logical file size [default: physical] [possible values: logical, physical]
  -f, --follow                     Follow symlinks
  -F, --flat                       Print disk usage information in plain format without the ASCII tree
  -H, --human                      Print disk usage in human-readable format
  -i, --no-ignore                  Do not respect .gitignore files
  -I, --icons                      Display file icons
  -l, --long                       Show extended metadata and attributes
      --octal                      Show permissions in numeric octal format instead of symbolic
      --time <TIME>                Which kind of timestamp to use; modified by default [possible values: created, accessed, modified]
  -L, --level <NUM>                Maximum depth to display
  -p, --pattern <PATTERN>          Regular expression (or glob if '--glob' or '--iglob' is used) used to match files
      --glob                       Enables glob based searching
      --iglob                      Enables case-insensitive glob based searching
  -t, --file-type <FILE_TYPE>      Restrict regex or glob search to a particular file-type [possible values: file, dir, link]
  -P, --prune                      Remove empty directories from output
  -s, --sort <SORT>                Sort-order to display directory content [default: size] [possible values: name, size, size-rev]
      --dirs-first                 Sort directories above files
  -T, --threads <THREADS>          Number of threads to use [default: 3]
  -u, --unit <UNIT>                Report disk usage in binary or SI units [default: bin] [possible values: bin, si]
  -., --hidden                     Show hidden files
      --no-git                     Disable traversal of .git directory when traversing hidden files
      --completions <COMPLETIONS>  Print completions for a given shell to stdout [possible values: bash, elvish, fish, powershell, zsh]
      --dirs-only                  Only print directories
      --inverted                   Print tree with the root directory at the topmost position
      --no-color                   Print plainly without ANSI escapes
      --no-config                  Don't read configuration file
      --suppress-size              Omit disk usage from output
      --truncate                   Truncate output to fit terminal emulator window
  -h, --help                       Print help (see more with '--help')
  -V, --version                    Print version
```

Of all the arguments, the following are not available on Windows:

```
  -l, --long                       Show extended metadata and attributes
      --octal                      Show permissions in numeric octal format instead of symbolic
      --time <TIME>                Which kind of timestamp to use; modified by default [possible values: created, accessed, modified]
```

## Installation

### crates.io (non-Windows)

Make sure you have [Rust and its toolchain](https://www.rust-lang.org/tools/install) installed.

```
$ cargo install erdtree
```

### crates.io (Windows)

The Windows version relies on some experimental features in order to properly support hard-link detection. If you want to build from `crates.io` you'll first need to install the nightly toolchain before installing `erdtree`:

```
$ rustup toolchain install nightly-2023-03-05         
```

Thereafter:

```
$ cargo +nightly-2023-03-05 install erdtree                    
```

### Homebrew-core

```
$ brew install erdtree
```

### Scoop

```
$ scoop install erdtree
```

### NetBSD

```
$ pkgin install erdtree
```

### Releases

Binaries for common architectures can be downloaded from latest releases.

### Latest non-release

If you'd like the latest features that are on `master` but aren't yet included as part of a release:

```
$ cargo install --git https://github.com/solidiquis/erdtree --branch master
```

Other means of installation to come.

## Documentation

### Configuration file

If `erdtree`'s out-of-the-box defaults don't meet your specific requirements, you can set your own defaults using a configuration file.

`erdtree` will look for a configuration file in any of these locations:
- `$ERDTREE_CONFIG_PATH`
- `$XDG_CONFIG_HOME/erdtree/.erdtreerc`
- `$XDG_CONFIG_HOME/.erdtreerc`
- `$HOME/.config/erdtree/.erdtreerc`
- `$HOME/.erdtreerc`

The format of a config file is as follows:
- Every line is an `erdtree` option/argument.
- Lines starting with `#` are considered comments and are thus ignored.

Arguments passed to `erdtree` take precedence. If you have a config that you would like to ignore without deleting you can use `--no-config`.

Here is an example of a valid configuration file:

```
# Long argument
--icons
--human

# or short argument
-l

# args can be passed like this
-d logical

# or like this
--unit=si
```

### Hardlinks

If multiple hardlinks that point to the same inode are in the same file-tree, both will be included in the output but only one is considered when computing overall disk usage.

### Symlinks (`-f, --follow`)

Symlinks will never be counted towards the total disk usage. When a symlink to a directory is followed all of its descendents will be painted a different

### Binary prefix or SI Prefix

Disk usage is reported using binary prefixes by default (e.g. `1 KiB = 1024 B`) as opposed to SI prefixes (`1 KB = 1000 B`). To toggle between the two use the `-p, --prefix` option.

### Logical or physical disk usage

Logical sizes are reported by default but you can toggle the reporting to physical sizes which takes into account compression, sparse files, and actual blocks allocated to a particular file via the following option:

```
-d, --disk-usage <DISK_USAGE>  Print physical or logical file size [default: logical] [possible values: logical, physical]
```

### How are directory sizes computed

- A directory will have a size equal to the sum of the sizes of all of its entries.
- Hidden files, files excluded by `.gitignore`, and files excluded via globbing will be omitted from the disk usages of their parent directories.
- Files/Directories that don't have read permissions will be omitted from the disk usages of their parent directories.
- Special files such a named pipes, sockets, etc. have negligible sizes so their disk usage aren't reported.

### Symlinks

- If symlink following is not enabled via `-S, --follow-links`, the disk usages of their target will not be reported nor considered.
- If symlink following is enabled the size of the target will be reported and considered as part of the total of the symlink's ancestral directories.
- The parts of the file-tree that branch from the symlink that's followed are printed in a different color.

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/follow_links_demo.png?raw=true" alt="failed to load png" />
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

### Completions

`--completions` is used to generate auto-completions for common shells so that the `tab` key can attempt to complete your command or give you hints; where you place the output highly depends on your shell as well as your setup. In my environment where I use `zshell` with `oh-my-zsh`, I would install completions like so:

```
$ et --completions zsh > ~/.oh-my-zsh/completions/_et
$ source ~/.zshrc
```

### Plain view

`-r, --report` offers a more traditional `du`-like view of disk usage info with the additional of file-type identifiers you'd expect on `ls -l` for POSIX systems or `Get-ChildItem` on Windows.

#### Regular view
<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/report.png?raw=true" alt="failed to load png" />
</p>

#### Human readable
<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/report_human.png?raw=true" alt="failed to load png" />
</p>

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

[dust](https://github.com/bootandy/dust) is another fantastic tool, but it's one that heavily overlaps with `erdtree` in functionality. On the surface you'll find that the biggest differences are the out-of-the-box defaults - but of course you can override `erdtree`'s defaults with a [config file](#configuration-file) if you so choose - as well as the UI.

On the topic of performance you'll find that there is negligible difference between the two. In the following crude benchmark the options supplied to `erdtree` make it mirror `dust` as closely as possible in behavior with the exception of icons.

#### dust
<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/dust_benchmark_v2.png?raw=true" alt="failed to load png" />
</p>

#### erdtree
<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/et_benchmark_v2.png?raw=true" alt="failed to load png" />
</p>

Ultimately you should give both tools a try and see which one best suits you :]

## Rules for Contributing and Feature Requests

Happy to accept contributions but please keep the following in mind:
- If you'd like to add a feature please open up an issue and receive approval first unless you've previously contributed. You can also start a [discussion](https://github.com/solidiquis/erdtree/discussions/49).
- If new arguments/options are added please do your best to keep them sensibly alphabetized. Also be sure to update the [Usage][#usage] section of the README.
- The code is heavily documented so please follow suit. `cargo doc --open` can be extremely helpful.
- Feature adds generally require tests.
- If no one is assigned to an `up for grabs` issue feel free to pick it up yourself :]

Feature requests in the form of issues in general are welcome.

## Special thanks

- to [luccahuguet](https://github.com/luccahuguet) on Github) for suggesting that the compiled `erdtree` binary be shorted to `et`.
- to [messense](https://github.com/messense) for getting this on Homebrew-core!
- to [fawni](https://github.com/fawni) for getting this on Scoop!
- to [0323pin](https://github.com/0323pin) for getting this on NetBSD!
- to all contributors :]

## Questions you might have

_Q: Why did you make this? It's totally unnecessary._

A: Ennui.

_Q: Why is it called erdtree?_

A: It's a reference to Elden Ring.

_Q: Is it any good?_

A: Yes.

_Q: Got any testimonials?_

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/tarnished_i.png?raw=true" alt="failed to load png" />
</p>

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/tarnished_ii.png?raw=true" alt="failed to load png" />
</p>

_Q: Is it blazingly fast?_

A: I wrote it in Rust so it should be blazingly fast.

<img src="https://i.redd.it/t7ns9qtb5gh81.jpg">
