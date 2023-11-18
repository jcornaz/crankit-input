# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]


## [0.3.1] - 2023-11-18

### Documentation

Attempt at fixing documentation on https://docs.rs/crankit-input


## [0.3.0] - 2023-11-18

### Breaking changes

`InputSystem` is removed and is replace by two *traits* `ButtonsStateSource` and `CrankStateSource`.

There are implementation of those traits for the types in [`playdate-sys`](https://docs.rs/playdate-sys/0.2) behind the feature
flag `playdate-sys-v02` (enabled by default).

`crank_angle` and `crank_change` methods are replaced by `crank_angle_rad`, `crank_angle_deg`, `crank_change_rad` and `crank_change_deg`,
for more explicit angle unit.

### Added

* Implement `From<ButtonSet>` for `playdate_sys::ffi::PDButtons`


### Documentation

* General documentation improvements, including fixes and more documentation.


## [0.2.0] - 2023-11-18

### Breaking changes

* Remove arithmetic operators (`+`, `+=`, `Sum`) implementation for `ButtonSet`

### Added

* `ButtonSet::insert`
* Implement `From<[Button; N]>` and `From<&[Button]>` for `ButtonSet`

### Documentation

* Minor documentation inprovements


## [0.1.0] - 2023-11-17

Provide the buttons states (pressed, just pressed, released)

Allow to get the D-Pad state as a vector

Provide crank state (angle, change and is-docked)


[Unreleased]: https://github.com/jcornaz/beancount_parser_2/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/jcornaz/beancount_parser_2/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/jcornaz/beancount_parser_2/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/jcornaz/beancount_parser_2/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jcornaz/crankit-input/compare/...v0.1.0
