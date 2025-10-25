# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- All-day event support with notifications the day before at midday
- Asynchronous sync functionality for background operations
- One file per event storage system
- Multi-day event support
- Event format specification documentation

### Changed
- Event files are now named by event title instead of date-based naming

### Fixed
- Fixed git text display in TUI
- Fixed issue with endless events
- Removed debug print "Pushing event" that was appearing in the TUI during event loading in sync popup