# Change Log
All notable changes to this project will be documented in this file.
 
This project adheres to [Semantic Versioning](http://semver.org/).
 
## [0.1.0] - 2022-05-08

First release.

## [1.0.0] - 2023-02-07

Did a complete rewrite with emphasis on an intuitive interface and performance. Notable changes:
- Binary renamed to `et` for brevity.
- Respects `.gitignore` and hidden file rules.
- Parallel filesystem traversal.
- Completely new CLI. `$ erdtree -h` for usage info.
- Uses `LS_COLORS` environment variable for file coloring.

## [1.1.0] - 2023-02-14

### What changed
- `-S, --follow-links` added to give option to traverse symlinks to directories. If enabled the disk usages of the target directory is considered.
- More test coverage.

### Contributors
- [jprochazk](https://github.com/jprochazk): [Filtering functionality via glob options](https://github.com/solidiquis/erdtree/pull/12)
- [tintin](https://github.com/Tlntin): [Cross-compilation and CI](https://github.com/solidiquis/erdtree/pull/18)
