# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

-  Common parts from `cortex-m-rfm` has been moved here

### Changed

- [breaking-change] The `Mutex` trait has been removed in favor of `mutex-trait` crate

## [v0.3.0] - 2019-11-14

### Added

- a `Exclusive` newtype over a mutable reference that implements the `Mutex`
  trait

### Changed

- [breaking-change] The `Resource` trait has been renamed to `Mutex` and no
  longer requires a `Threshold` argument

### Removed

- [breaking-change] removed the `Threshold` struct

## [v0.2.0] - 2018-01-15

### Changed

- drop dependency on the unstable `optin_builtin_traits` feature
- [breaking-change] drop the `Static` wrapper from the `Resource` API.

## v0.1.0 - 2017-07-29

Initial release

[Unreleased]: https://github.com/rtfm-rs/rtfm-core/compare/v0.3.0...HEAD
[v0.3.0]: https://github.com/rtfm-rs/rtfm-core/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/rtfm-rs/rtfm-core/compare/v0.1.0...v0.2.0
