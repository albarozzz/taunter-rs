
# Change Log
All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).
 
## [Unreleased]
 
### Added

### Changed

## [v0.4.2] - 2022-11-30
 
To update the taunter you just have to overwrite the binary!
**IMPORTANT: to run the program from a configuration file `./taunter --config config.json`**
 
### Added

- Added a CHANGELOG.md to the project.

### Changed

- Changed release.yml to include a `config.json` in releases.
- Modified conditional in check() when `regex` feature is enabled.

## [v0.4.1] - 2022-11-30
  
To update the taunter you just have to overwrite the binary!
**IMPORTANT: to run the program from a configuration file `./taunter --config config.json`**

### Added

- Added CLI support, make sure to check it out using `./taunter --help`!
- Regex support via feature if you want to test it use `cargo build --release --features regex`, it may be dangerous as it uses look-arounds, positive look-aheads and things like that but it enables making username_victim an array of strings if you want to target several enemies.

### Changed
  
- Refactored all functions in main.rs to helper.rs
- Bumped to version 0.4.1