# Erdtree
A bLazInGlY fAsT, simplified version of the ancient [tree](https://en.wikipedia.org/wiki/Tree_(command)) command which displays a colorful depth indented listing of files
with their memory sizes adjacent.

<img src="https://github.com/solidiquis/solidiquis/blob/master/assets/Screen%20Shot%202022-05-04%20at%2011.31.21%20PM.png?raw=true">

## Usage
```
Usage:
    erdtree [options]

OPTIONS:
-d        Directory to traverse. Defaults to current working directory.
-l        Unsigned integer indicating many nested directory levels to display. Defaults to all.
-p        Comma-separated list of prefixes. Directories containing any of
          these prefixes will not be traversed. Their memory size will also be ignored.
-h        Displays help prompt.
```

## Installation

### Cargo

1. Make sure you have [Rust and its toolchain](https://www.rust-lang.org/tools/install) installed.
2. `$ cargo install --git https://github.com/solidiquis/erdtree`
3. The executable should then be located in `$HOME/.cargo/bin/`.

### Manual Installation

`todo!()`

### Brew

`todo!()`

## Questions you might have

_Q: Why did you make this? It's totally unnecessary._

A: I had two six-hour flights and got bored.

_Q: Is it any good?_

A: Yes.

_Q: How do you know that this is blazingly fast?_

A: I wrote it in Rust.

<img src="https://i.redd.it/t7ns9qtb5gh81.jpg">
