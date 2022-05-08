# Change Log
All notable changes to this project will be documented in this file.
 
This project adheres to [Semantic Versioning](http://semver.org/).
 
## [1.0.0] - 2021-05-07

### New

[Feature request](https://github.com/solidiquis/erdtree/issues/3)

Added option `-s <asc|desc>` to sort files by memory size.

### Other changes

- Directory now passed as positional argument as opposed to `-d` option. This is the breaking change that warranted major version bump.
- Modified help text that gets displayed when passing `-h` option.

## [0.0.2] - 2021-05-05
  
### Fixed
 
Issue: [Error when directory contains too much directories and files](https://github.com/solidiquis/erdtree/issues/2)

([f6803e0](https://github.com/solidiquis/erdtree/commit/f6803e081929789d75f1974110c3c22cfa7ad87b)) Patch panics that occur when files were missing user read permissions and when symlinks point to non-existent files.
 
## [0.0.1] - 2021-05-04
 
First release.
