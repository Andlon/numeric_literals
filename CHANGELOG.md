# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.2] - 2021-07-06
### Added
 - An optional macro parameter `visit_macros` was added to control whether literals inside of macro invocations should be replaced. See the readme or docs for an example.
### Changed
 - By default, literals are now also replaced inside of macro invocations
### Fixed
 - Integer literals with float suffixes (e.g. `1_f64`) are now correctly identified as floats and replaced accordingly

## [0.1.1] - 2019-11-26

### Internal
 - Updated `syn` and `quote` to version 1.0

## [0.1] - 2021-05-19

Initial release.