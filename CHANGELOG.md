# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]

### Added

* `InputSystem::crank_angle_rad` and `InputSystem::crank_change_rad` to get crank state in radians
* `InputSystem::crank_angle_deg` and `InputSystem::crank_change_deg` to explicitely get crank state in degees
* Implement `From<ButtonSet>` for `playdate_sys::ffi::PDButtons`

### Deprecated

* `InputSystem::crank_angle` and `InputSystem::crank_change`: One should explicitely choose the angle unit by calling the respective `*_deg` or `*_rad` method instead.

### Documentation

* minor documentation fixes


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


[Unreleased]: https://github.com/jcornaz/beancount_parser_2/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/jcornaz/beancount_parser_2/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jcornaz/crankit-input/compare/...v0.1.0
