
# Change Log
All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).
 
## [Unreleased]

## [v0.6.0] - 2022-12-18

### Added

- Added a necessary step to README.md ('log on' on autoexec.cfg).
- Added 2 tests to ensure that it works (at least check()).
- Added conditional compiling to soundpad functions because soundpad is only in windows.

### Changed

- Changed check() to match a regex.
- Changed shortcut of argument ignore warning from 'g' to 'i'.
- Changed descriptions of some arguments.

### Fixed

- Fixed implementation of read_to_string() to stop reading huge files.

### Removed

- Deleted useless borrow in Path::new().

## [v0.5.0] - 2022-11-30
 
To update the taunter you just have to overwrite the binary!
**IMPORTANT: to run the program from a configuration file `./taunter --config config.json`**
 
### Added

- Changed check() to analyze the string and extract the username and the victim's name.
- Now username_victim is a list of string and not a string.

### Removed

- Removed all regex from the program.

## [v0.4.3] - 2022-11-30
 
To update the taunter you just have to overwrite the binary!
**IMPORTANT: to run the program from a configuration file `./taunter --config config.json`**
 
### Added

- Added an example configuration file config.json.

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