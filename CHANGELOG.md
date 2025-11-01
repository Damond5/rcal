# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2025-11-01

### Added
- All-day event support with notifications the day before at midday
- Asynchronous sync functionality for background operations
- Added automatic pull on launch when sync is configured, performed asynchronously in the background
- One file per event storage system
- Multi-day event support
- Event format specification documentation

### Changed
- Event files are now named by event title instead of date-based naming
- Removed ID field from CalendarEvent struct, using title as unique identifier for file operations
