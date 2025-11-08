# tui-architecture Specification

## Purpose
TBD - created by archiving change add-agent-guidelines-specs. Update Purpose after archive.
## Requirements
### Requirement: Popup Layout Design
Popups MUST use centered, fixed-size rectangles for consistent positioning.

#### Scenario: Popup Centering
Given a popup display,
When using Rect::new() for positioning,
Then popup is centered on screen.

### Requirement: Input Handling Implementation
Input MUST use custom cursor management with character-based indexing for Unicode support.

#### Scenario: Unicode Cursor Movement
Given text input with Unicode characters,
When moving cursor,
Then character boundaries are respected.

### Requirement: State Management Pattern
Application MUST use enum-based input modes for clear state transitions including confirmation dialogs.

#### Scenario: Mode Transition
Given user actions,
When changing input modes,
Then state transitions are clear.

### Requirement: Event Filtering Mechanism
Events MUST be filtered by date using iterator methods for efficient display.

#### Scenario: Date-Based Filtering
Given events list,
When filtering by date,
Then only relevant events display.

### Requirement: Modal Interaction Support
Application MUST support nested popup states with proper state restoration.

#### Scenario: Nested Popups
Given view → add → confirm sequence,
When navigating popups,
Then state is properly restored.

### Requirement: Keybinding Preferences
Keybindings MUST prefer vim-like navigation (h/j/k/l) over modifier-based shortcuts.

#### Scenario: Vim Navigation
Given navigation actions,
When using h/j/k/l keys,
Then movement works as expected.

### Requirement: Keybind Hint Display
Contextual keybind hints MUST be displayed on all screens and popups as footer.

#### Scenario: Hint Visibility
Given any screen or popup,
When displaying interface,
Then keybind hints appear at bottom.

### Requirement: Cycling Navigation
In events view popup, up/down navigation MUST cycle to opposite end when reaching bounds.

#### Scenario: Boundary Cycling
Given event list navigation,
When reaching top/bottom,
Then cycles to opposite end.

