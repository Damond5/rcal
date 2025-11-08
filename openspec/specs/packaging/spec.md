# packaging Specification

## Purpose
TBD - created by archiving change add-agent-guidelines-specs. Update Purpose after archive.
## Requirements
### Requirement: PKGBUILD Structure
Package MUST use standard Rust PKGBUILD with `cargo build --release`.

#### Scenario: Release Build
Given source code,
When building package,
Then uses cargo build --release.

### Requirement: Binary Installation
Binary MUST be installed to `/usr/bin/rcal`.

#### Scenario: Binary Placement
Given built binary,
When installing,
Then places in /usr/bin/rcal.

### Requirement: License Installation
License MUST be installed to `/usr/share/licenses/rcal/`.

#### Scenario: License Placement
Given LICENSE file,
When packaging,
Then installs to standard location.

### Requirement: Systemd Service Installation
Systemd user service MUST be installed to `/usr/lib/systemd/user/rcal.service`.

#### Scenario: Service File Placement
Given rcal.service,
When packaging,
Then installs systemd user service.

### Requirement: Systemd User Service
Package MUST provide systemd user service for daemon mode.

#### Scenario: Daemon Enablement
Given installed package,
When enabling service,
Then daemon runs via systemctl --user enable rcal.service.

### Requirement: Manual Service Enabling
Service enabling MUST be manual by users to avoid interference.

#### Scenario: User Control
Given package installation,
When not auto-enabling service,
Then user decides when to enable.

### Requirement: Minimal Dependencies
Package MUST have no runtime dependencies, only build deps (cargo, rust).

#### Scenario: Dependency Management
Given package definition,
When specifying deps,
Then only build-time dependencies listed.

### Requirement: Source Repository
Package MUST use GitHub git repo with release tags for versioning.

#### Scenario: Version Tracking
Given upstream releases,
When updating package,
Then uses tagged versions.

### Requirement: Package Maintenance
Package MUST be updated with new pkgver and checksums on upstream releases.

#### Scenario: Version Updates
Given new release,
When maintaining package,
Then updates pkgver and checksums.

