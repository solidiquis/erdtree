# erdtree (erd)

[![Build status](https://github.com/solidiquis/erdtree/actions/workflows/ci.yml/badge.svg)](https://github.com/solidiquis/erdtree/actions)
[![Crates.io](https://img.shields.io/crates/v/erdtree.svg)](https://crates.io/crates/erdtree)
[![Packaging status](https://repology.org/badge/tiny-repos/erdtree.svg)](https://repology.org/project/erdtree/versions)
[![Crates.io](https://img.shields.io/crates/d/erdtree)](https://crates.io/crates/erdtree)

`erdtree` is a modern, cross-platform, and multi-threaded filesystem and disk-usage analysis tool. The following are some feature highlights:
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
  - [Hardlinks](#hardlinks)
  - [Symlinks](#symlinks)
  - [Disk usage](#disk-usage)
  - [Flat view](#flat-view)
  - [gitignore](#gitignore)
  - [Hidden files](#hidden-files)
  - [Icons](#icons)
  - [Maximum depth](#maximum-depth)
  - [Pruning empty directories](#pruning-empty-directories)
  - [Sorting](#sorting)
  - [Directories only](#directories-only)
  - [Permissions](#permissions)
  - [Regular expressions and globbing](#regular-expressions-and-globbing)
  - [Truncating output](#truncating-output)
  - [Redirecting output and colorization](#redirecting-output-and-colorization)
  - [Parallelism](#parallelism)
  - [Completions](#completions)
* [Comparisons against similar programs](#comparisons-against-similar-programs)
  - [exa](#exa)
  - [dua](#dua)
  - [dust](#dust)
  - [fd](#fd)
* [Rules for contributing](#rules-for-contributing)
* [Security policy](#security-policy)
* [Questions you might have](#questions-you-might-have)

## Usage

```
erdtree (erd) is a cross-platform multi-threaded filesystem and disk usage analysis tool.

Usage: erd [OPTIONS] [DIR]

Arguments:
  [DIR]  Directory to traverse; defaults to current working directory

Options:
  -C, --color <COLOR>              Mode of coloring output [default: auto] [possible values: none, auto, forced]
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
      --dir-order <DIR_ORDER>      Sort directories before or after all other file types [default: none] [possible values: none, first, last]
  -T, --threads <THREADS>          Number of threads to use [default: 3]
  -u, --unit <UNIT>                Report disk usage in binary or SI units [default: bin] [possible values: bin, si]
  -., --hidden                     Show hidden files
      --no-git                     Disable traversal of .git directory when traversing hidden files
      --completions <COMPLETIONS>  Print completions for a given shell to stdout [possible values: bash, elvish, fish, powershell, zsh]
      --dirs-only                  Only print directories
      --inverted                   Print tree with the root directory at the topmost position
      --no-config                  Don't read configuration file
      --suppress-size              Omit disk usage from output
      --truncate                   Truncate output to fit terminal emulator window
  -h, --help                       Print help (see more with '--help')
  -V, --version                    Print version
```

Of all the above arguments, the following are not yet available on Windows but will be in the near future:

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

On Linux/Mac/Unix-like:
- `$ERDTREE_CONFIG_PATH`
- `$XDG_CONFIG_HOME/erdtree/.erdtreerc`
- `$XDG_CONFIG_HOME/.erdtreerc`
- `$HOME/.config/erdtree/.erdtreerc`
- `$HOME/.erdtreerc`

On Windows:
- `$ERDTREE_CONFIG_PATH`
- `%APPDATA%/erdtree/.erdtreerc`

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

If multiple hardlinks that point to the same inode are in the same file-tree, all will be included in the output but only one is considered when computing overall disk usage.

### Symlinks

```
-f, --follow                     Follow symlinks
```

Symlinks will never be counted towards the total disk usage. When a symlink to a directory is followed all of the box-drawing characters of its descendants will
be painted in a different color for better visual feedback:

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/symlinks.png?raw=true" alt="failed to load picture" />
</p>

### Disk usage

Disk usage is reported in total amount of bytes by default but can output in a human readable format using:

```
-H, --human                      Print disk usage in human-readable format
```

#### Regular format:

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/inhuman_readable.png?raw=true" alt="failed to load picture" />
</p>

#### Human-readable format:

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/h_readable.png?raw=true" alt="failed to load picture" />
</p>

Additionally, disk usage is reported using binary prefixes by default (e.g. `1 KiB = 1024 B`) but SI prefixes can be used as well (`1 KB = 1000 B`) using:

```
-u, --unit <UNIT>                Report disk usage in binary or SI units [default: bin] [possible values: bin, si]
```

Furthermore, physical size which takes into account compression, sparse files, and actual blocks allocated to a particular file are used by default.
Logical size which just reports the total number of bytes in a file may also be used.

```
-d, --disk-usage <DISK_USAGE>    Print physical or logical file size [default: physical] [possible values: logical, physical]
```

Lastly, if you'd like to omit disk usage from the output:

```
--suppress-size              Omit disk usage from output
```

### Flat view

```
-F, --flat                       Print disk usage information in plain format without the ASCII tree
```

For a more traditional `du`-like view without the ASCII tree, use `-F, --flat`.

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/inhuman_readable_flat.png?raw=true" alt="failed to load picture" />
</p>

#### Human readable disk usage
<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/human_readable_flat.png?raw=true" alt="failed to load picture" />
</p>

#### Human readable and long view
<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/flat_human_long.png?raw=true" alt="failed to load picture" />
</p>

### gitignore

```
-i, --no-ignore                  Do not respect .gitignore files
```

`.gitignore` is respected by default but can be disregarded with the above argument. `.gitignore` rules are also respected on a per directory basis, so
every directory that is encountered during traversal that has a `.gitignore` will also be considered.

If `.gitignore` is respected any file that is ignored will not be included in the total disk usage.

### Hidden files

```
-., --hidden                     Show hidden files
    --no-git                     Disable traversal of .git directory when traversing hidden files
```

Hidden files ignored by default but can be included with `-., --hidden`. If opting in to show hidden files `.git` is included; to exclude
it use `--no-git`.

If hidden files are ignored it will not be included in the total disk usage.

### Icons

```
-I, --icons                      Display file icons
```

Icons are an opt-in feature because for icons to render properly it is required that the font you have hooked up to your terminal emulator contains the glyphs necessary to properly render icons.

If your icons look something like this:

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/broken_icons.png?raw=true" alt="failed to load png" />
</p>

this means that the font you are using doesn't include the relevant glyphs. To resolve this issue download a [NerdFont](https://www.nerdfonts.com/) and hook it up to your terminal emulator.

### Maximum depth

Directories are fully traversed by default. To limit the maximum depth:

```
-L, --level <NUM>                Maximum depth to display
```

Limiting the maximum depth to display will not affect the total disk usage report.

### Pruning empty directories

Sometimes empty directories may appear in the output. To remove them:

```
-P, --prune                      Remove empty directories from output
```

### Sorting

Various sorting methods are provided:

```
-s, --sort <SORT>                Sort-order to display directory content [default: size] [possible values: name, size, size-rev]
    --dir-order <DIR_ORDER>      Sort directories before or after all other file types [default: none] [possible values: none, first, last]
```

To add extra granularity to how directories are sorted relative to other file-types, use `--dir-order`:

```
--dir-order <DIR_ORDER>
    Sort directories before or after all other file types
    
    [default: none]

    Possible values:
    - none:  Directories are ordered as if they were regular nodes
    - first: Sort directories above files
    - last:  Sort directories below files
```

### Directories only

You output only directories with:

```
--dirs-only                  Only print directories
```

This will not affect total disk usage.

### Permissions

Unix file permissions as well as metadata associated with directory entries can be shown using the following:

```
-l, --long                       Show extended metadata and attributes
    --octal                      Show permissions in numeric octal format instead of symbolic
    --time <TIME>                Which kind of timestamp to use; modified by default [possible values: created, accessed, modified]
```

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/showcase_top.png?raw=true" alt="failed to load picture" />
</p>

The columns shown in the order of left to right are:
  * inode number
  * [permissions](https://en.wikipedia.org/wiki/File-system_permissions#Notation_of_traditional_Unix_permissions)
  * The number of hardlinks of the underlying inode
  * The number of blocks allocated to that particular file
  * The date the file was last modified (or created or last accessed)

File permissions are currently not supported for Windows but will be sometime in the near future.

Additionally, although the permissions column does indicate whether or not extended attributes exist on a particular file, showing the extended
attributes will not be supported.

### Regular expressions and globbing

Searching for a particular file using a regular expression or glob is supported using the following:

```
-p, --pattern <PATTERN>          Regular expression (or glob if '--glob' or '--iglob' is used) used to match files
    --glob                       Enables glob based searching
    --iglob                      Enables case-insensitive glob based searching
-t, --file-type <FILE_TYPE>      Restrict regex or glob search to a particular file-type [possible values: file, dir, link]
```

Note that applying the regular expression or glob to a particular file-type is supported; regular files are the default file-type.

Additionally, any file that is filtered out will be excluded from the total disk usage.

Lastly, when applying a regular expression or glob to directories, off its descendants regardless of file-type will be included in the output.
If you desire to only show directories you may use `--dirs-only`.

References:
  * [Globbing rules](https://git-scm.com/docs/gitignore#_pattern_format)
  * [Regular expressions](https://docs.rs/regex/latest/regex/#syntax)

### Truncating output

In instances where the output does not fit the terminal emulator's window, the output itself may be rendered incoherently:

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/untruncated.png?raw=true" alt="failed to load picture" />
</p>

In these situations the following may be used:

```
--truncate                   Truncate output to fit terminal emulator window
```

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/truncated.png?raw=true" alt="failed to load picture" />
</p>

### Redirecting output and colorization

If you wish to force a colorless output the following may be used:
```
-C none         Print plainly without ANSI escapes
```

Colorization is also turned off if the output is redirected to something that is not a tty. If you wish to preserve the ANSI escape sequences (e.g.
preserve the colors as in the case of piping) the following may be used:

```
-C, forced                Turn on colorization always
```

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/no_color.png?raw=true" alt="failed to load picture" />
</p>

<p align="center">
  <img src="https://github.com/solidiquis/erdtree/blob/master/assets/force_color.png?raw=true" alt="failed to load picture" />
</p>

### Parallelism

The amount of threads used by `erdtree` can be adjusted with the following:

```
-T, --threads <THREADS>          Number of threads to use [default: 3]
```

#### Why parallelism

A common question that gets asked is how parallelism benefits disk reads when filesystem I/O is processed serially.

While this is true, parallelism still results in improved throughput due to the fact that disks have a [queue depth](https://en.wikipedia.org/wiki/IOPS)
that, when saturated, allows requests to be processed in aggregate keeping the disk busy as opposed to having it wait on `erdtree` to do CPU-bound processing
in between requests. Additionally these threads aren't just parallelizing disk reads, they're also parallelizing the processing of the data that is ultimately retrieved.

It should be noted however that the performance as a function of thread-count is asymptotic in nature (see [Amdahl's Law](https://en.wikipedia.org/wiki/Amdahl%27s_law))
so you'll quickly reach a point of dimishing returns after a certain thread-count threshold as you'd be paying the cost of managing a larger threadpool with no added benefit.

For empirical data on the subject checkout [this article](https://pkolaczk.github.io/disk-parallelism/).

### Completions

`--completions` is used to generate auto-completions for common shells so that the `tab` key can attempt to complete your command or give you hints; where you place the output highly depends on your shell as well as your setup. In my environment where I use `zshell` with `oh-my-zsh`, I would install completions like so:

```
$ et --completions zsh > ~/.oh-my-zsh/completions/_erd
$ source ~/.zshrc
```

## Rules for contributing

For rules on how to contribute please refer to [CONTRIBUTING.md](CONTRIBUTING.md).

## Security policy

For information regarding `erdtree`'s security policy and how to report a security vulnerability please refer to [SECURITY_POLICY.md](SECURITY.md)

## Comparisons against similar programs

It goes without saying that the following programs are all amazing in their own right and were highly influential in `erdtree`'s development. While each of the following are highly
specialized in acting as modern replacements for the more traditional Unix commands that we all know and love, `erdtree` aims to take bits and pieces of
them and their ancestors that people might use most frequently and assembles them into a unified highly practical tool.

No case will be made as to why `erdtree` should be preferred over X, Y, or Z, but because of some notable similarities with the following programs it is worth a brief
comparison.

### [exa](https://github.com/ogham/exa)

`exa` and `erdtree` are similar in that they both have tree-views and display information about file-permissions.

As it stands, however, `exa` [does not provide information about the disk usages of directories](https://github.com/ogham/exa/issues/91) which makes sorting files by
size a little dubious.

The advantage `exa` has over `erdtree`, however, is in the fact that `exa` is much more comprehensive as an `ls` replacement. `erdtree`
does not share this goal.

Both tools are complimentary to one another and it is encouraged that you have both in your toolkit.

### [dua](https://github.com/Byron/dua-cli)

`dua` is a fantastic interactive disk usage tool that serves as a modern alternative to
[ncdu](https://en.wikipedia.org/wiki/Ncdu). If you're in the mood for something interactive and solely focused on disk usage then `dua` might suit you more.
If you're interested in file permissions and doing quick static analysis of your disk usage without spinning up an entire interactive UI then perhaps consider `erdtree`.

### [dust](https://github.com/bootandy/dust)

`dust` is another fantastic tool that is closer in geneology to the traditional `du` command. If you're strictly looking for
a modern replacement to `du` then `dust` is a great choice.

### [fd](https://github.com/sharkdp/fd)

`fd` is much more comprehensive as a general finder tool, offering itself as a modern replacement to `find`. If you're looking for more granularity in your ability to search beyond just globbing, regular
expressions, and the three basic file types (files, directories, and symlinks) then `fd` is the optimal choice.

## Questions you might have

_Q: Why did you make this? It's totally unnecessary._

A: Ennui.

_Q: Why is it called erdtree?_

A: It's a reference to an object of worship in Elden Ring.

_Q: Is it any good?_

A: Yes.

_Q: Why is there no mention of this project being blazingly fast or written in Rust? Is it slow or something?_

A: Okay fine. `erdtree` is written in Rust and is blazingly fast.

<img src="https://i.redd.it/t7ns9qtb5gh81.jpg">
