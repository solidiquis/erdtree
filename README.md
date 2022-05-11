# Erdtree
A bLazInGlY fAsT, simplified version of the ancient [tree](https://en.wikipedia.org/wiki/Tree_(command)) command which displays a colorful depth indented listing of files
with their memory sizes adjacent.

<img src="https://github.com/solidiquis/solidiquis/blob/master/assets/Screen%20Shot%202022-05-04%20at%2011.31.21%20PM.png?raw=true">

## Usage
```
Usage:
    erdtree [directory] [options]

ARGUMENTS:
    directory     Directory to traverse. Defaults to current working directory.

OPTIONS:
    -l            Unsigned integer indicating many nested directory levels to display. Defaults to all.
    -p            Comma-separated list of prefixes. Directories containing any of
                  these prefixes will not be traversed. Their memory size will also be ignored.
    -s [asc|desc] Sort tree by memory-size. 
    -h            Displays help prompt.
```

## Installation

### Cargo

1. Make sure you have [Rust and its toolchain](https://www.rust-lang.org/tools/install) installed.
2. `$ cargo install --git https://github.com/solidiquis/erdtree`
3. The executable should then be located in `$HOME/.cargo/bin/`.

### Manual Installation

Download the binaries for your appropriate architecture from [the releases section](https://github.com/solidiquis/erdtree/releases). Currently available are binaries for Darwin systems only.

### Brew

```
$ brew tap solidiquis/tap
$ brew install erdtree
```

## Disambiguation about units for memory

As recommended in [IEC 80000-13](https://en.wikipedia.org/wiki/ISO/IEC_80000#cite_note-80000-13:2008-14), this utility will report memory sizes
using SI units rather than binary units. As such you can expect `1KB = 1000B` and not `1KiB = 1024B`.

## Questions you might have

_Q: Why did you make this? It's totally unnecessary._

A: I had two six-hour flights and got bored.

_Q: Is it any good?_

A: Yes.

_Q: How do you know that this is blazingly fast?_

A: I wrote it in Rust.

<img src="https://i.redd.it/t7ns9qtb5gh81.jpg">

## Actual benchmarks

This is not a rigorous way to perform benchmarks but here is how `erdtree` compares with `tree` in traversing a directory that is 3.5GB in size. Please note that `erdtree` is not a 1-to-1 port of `tree` as tree comes with many more sophisticated features that I felt most wouldn't use, but I've gotten enough interest that warranted this rough comparison.

Erdtree:
```
$ time erdtree >> /dev/null
erdtree >> /dev/null  0.35s user 1.55s system 99% cpu 1.918 total
$ time erdtree >> /dev/null
erdtree >> /dev/null  0.35s user 0.90s system 99% cpu 1.255 total
$ time erdtree >> /dev/null
erdtree >> /dev/null  0.35s user 0.89s system 99% cpu 1.253 total
```

Tree:
```
$ time tree >> /dev/null
tree >> /dev/null  0.64s user 1.04s system 99% cpu 1.690 total
$ time tree >> /dev/null
tree >> /dev/null  0.63s user 1.03s system 99% cpu 1.669 total
$ time tree >> /dev/null
tree >> /dev/null  0.63s user 1.05s system 97% cpu 1.719 total
```
