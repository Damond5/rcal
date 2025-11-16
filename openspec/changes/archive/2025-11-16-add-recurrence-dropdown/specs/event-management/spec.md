## ADDED Requirements
### Requirement: Recurrence Dropdown Selection
The event creation/editing popup MUST provide a selectable dropdown for recurrence options instead of free-text input when the recurrence field is active.

#### Scenario: Dropdown Activation
Given the recurrence field is selected via Tab navigation,
When the field becomes active,
Then display a list of valid recurrence options (none, daily, weekly, monthly, yearly) for selection.

#### Scenario: Keyboard Navigation
Given the recurrence dropdown is active,
When user presses j/k keys,
Then highlight the next/previous option in the list.

#### Scenario: Selection Confirmation
Given a recurrence option is highlighted,
When user presses Enter,
Then set the recurrence field to the selected option and return to normal editing mode.

#### Scenario: Selection Cancellation
Given the recurrence dropdown is active,
When user presses Esc,
Then keep the recurrence field unchanged and return to normal editing mode.

### Requirement: Invalid Recurrence Prevention
Event creation MUST only accept valid recurrence values, preventing user errors from free-text input.

#### Scenario: Dropdown Enforcement
Given event creation with recurrence dropdown,
When saving the event,
Then only predefined options are possible, eliminating invalid recurrence strings.</content>
<parameter name="filePath">openspec/changes/add-recurrence-dropdown/specs/event-management/spec.md