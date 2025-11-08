# ui-rendering Specification

## Purpose
TBD - created by archiving change add-agent-guidelines-specs. Update Purpose after archive.
## Requirements
### Requirement: Popup Rendering Technique
Popups MUST use `ratatui::widgets::Clear` to render over calendar background.

#### Scenario: Background Clearing
Given popup display,
When using Clear widget,
Then background is properly obscured.

### Requirement: Input Field Presentation
Input fields MUST be wrapped in `Block` widgets with titles for identification.

#### Scenario: Field Labeling
Given input fields,
When wrapped in Block with titles,
Then fields are clearly identified.

### Requirement: Cursor Positioning Logic
Cursor coordinates MUST be calculated relative to input field positions using character-based indexing.

#### Scenario: Accurate Cursor Placement
Given text input,
When calculating cursor position,
Then it aligns with character position.

### Requirement: Style Management System
Styling MUST be conditional based on `selected_input_field` for visual feedback.

#### Scenario: Field Highlighting
Given multiple input fields,
When one is selected,
Then it shows visual distinction.

### Requirement: Event Selection Display
Selected events in view popup MUST be highlighted with black text on light blue background.

#### Scenario: Event Highlighting
Given selected event,
When displaying in popup,
Then it uses specified colors for readability.

### Requirement: Text Field Sizing
Text field heights MUST be fixed in event creation/editing popup for consistent visibility.

#### Scenario: Consistent Heights
Given event popup,
When displaying text fields,
Then heights remain fixed.

