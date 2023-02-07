# Erdtree (et)
A modern, vibrant (but not overly), and multi-threaded file-tree visualizer and disk usage analyzer that respects hidden files and `.gitignore` rules - basically if `tree` and `du` had a baby.

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/demo_erdtree.gif" alt="failed to load gif" />
</p>

**Erdtree** is a modern alternative to the ancient [tree](https://en.wikipedia.org/wiki/Tree_(command)) command in that it:
- offers a minimal and user-friendly CLI
- respects hidden files and `.gitignore` rules by default
- displays file sizes in human-readable format by default
- traverses directories in a parallel manner (4 threads by default)
- displays files using ANSI colors by default


## Usage
```
$ et -h
File tree visualizer and disk usage analyzer.

Usage: erdtree [OPTIONS] [DIR]

Arguments:
  [DIR]  Root directory to traverse; defaults to current working directory

Options:
  -i, --ignore-git-ignore      Ignore .gitignore; disabled by default
  -l, --level <NUM>            Maximum depth to display
  -t, --threads <THREADS>      Number of threads to use [default: 4]
  -s, --sort <SORT>            Sort-order to display directory content [default: none] [possible values: filename, size, none]
  -H, --hidden                 Whether to show hidden files; disabled by default
  -g, --glob <GLOB>            Include or exclude files using glob patterns
      --iglob <IGLOB>          Include or exclude files using glob patterns; case insensitive
      --glob-case-insensitive  Process all glob patterns case insensitively
  -h, --help                   Print help (see more with '--help')
  -V, --version                Print version
```

## Installation

### Cargo

1. Make sure you have [Rust and its toolchain](https://www.rust-lang.org/tools/install) installed.
2. `$ cargo install --git https://github.com/solidiquis/erdtree`
3. The executable should then be located in `$HOME/.cargo/bin/`.

Other means of installation to come.

## Disambiguations

### Disk Size

As recommended in [IEC 80000-13](https://en.wikipedia.org/wiki/ISO/IEC_80000#cite_note-80000-13:2008-14), this command will report sizes
using SI units rather than binary units. As such you can expect `1KB = 1000B` and not `1KiB = 1024B`. 

Additionally:
- A directory will have a size equal to the sum of the sizes of all of its entries. The size of the directory itself is negligble and isn't taken into account.
- Files other than directories and regular files (symbolic links, named pipes, sockets, etc.) appear but their memory sizes are not reported.
- Symbolic links to directories appear but are not traversed; their sizes are also not reported

### Files Without Read Permissions

Files that don't have read persmissions will appear but won't have their disk sizes reported. If they are directories they will not be traversed. Additionally, their size will not be included in their parent directory's total.

### File Coloring

Files are printed in ANSI colors specified according to the `LS_COLORS` environment variable on GNU/Linux systems. In its absence [a default value](https://docs.rs/lscolors/latest/src/lscolors/lib.rs.html#221) is used.

**Note for MacOS**: MacOS uses the `LSCOLORS` environment variable to determine file colors for the `ls` command which is formatted very differently from `LS_COLORS`. MacOS systems will fall back on the aforementioned default value unless the user defines their own `LS_COLORS` environment variable.

### `tree` command

This is not a rewrite of the `tree` command thus it should not be considered a 1-to-1 port. The basic idea is the same: Display the file-tree of the specified directory. There are, however, key fundamental differences under the hood with regard to how file sizes are computed, traversal method, hidden files and `.gitignore` rules, and printing.

### Advantages over `exa --tree`

[Exa](https://github.com/ogham/exa) is a powerful modern equivalent of the `ls` command which gives the option to print a tree-view of a specified directory, however the primary differences between `exa --tree` and `et` are:
- `exa --tree --git-ignore` doesn't respect `.gitignore` rules on a per directory basis whereas `et` does. With `exa` the root's `.gitignore` is considered, but if child directories have their own `.gitignore` they are disregarded and all of their contents will be printed.
- `et` displays the total size of a directory as the sum of all of its file sizes whereas `exa` [does not support this](https://github.com/ogham/exa/issues/91). This makes sorting directories in the tree-view by size dubious and unclear. Below are screenshots comparing equivalent usages of `et` and `exa`, using long option names for clarity.

#### Exa
<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/exa.png" alt="failed to load png" />
</p>

#### Erdtree
<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/erdtree.png" alt="failed to load png" />
</p>

## Rules for Contributing and Feature Requests

Happy to accept contributions but please keep the following in mind:
- If you're doing some minor refactoring and/or code cleanup feel free to just submit a PR.
- If you'd like to add a feature and/or make fundamental changes to `et`'s [traverse](https://github.com/solidiquis/erdtree/blob/e7f37d416d6d61b1d62e2200935b4813aaeab461/src/fs/erdtree/tree/mod.rs#L63) algorithm please open up an issue and get my approval first.
- Feature adds require tests.

Feature requests in the form of issues in general are welcome.

## Questions you might have

_Q: Why did you make this? It's totally unnecessary._

A: Ennui.

_Q: Why is it called Erdtree?_

A: It's a reference to Elden Ring.

_Q: Is it any good?_

A: Yes.

_Q: Is it blazingly fast?_

A: Should be. I wrote it in Rust.

<img src="https://i.redd.it/t7ns9qtb5gh81.jpg">
