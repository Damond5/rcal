## Context
The current recurrence input is a free-text field that accepts "none", "daily", "weekly", "monthly", "yearly" but provides no guidance to users. Invalid inputs default to "none" silently, which can be confusing.

## Goals / Non-Goals
- Goals: Provide intuitive recurrence selection, prevent invalid inputs, maintain TUI consistency
- Non-Goals: Support custom recurrence patterns, change existing parsing logic, add mouse support

## Triggering the Dropdown
On Tab navigation to the Recurrence field in `EditingEventPopup`, automatically switch to `SelectingRecurrence` mode. Disable character input for Recurrence to enforce dropdown usage.

## Pre-Selection Logic
When entering `SelectingRecurrence`, map `popup_event_recurrence` to `selected_recurrence_index` (0=none, 1=daily, etc.). If invalid, default to 0.

## Decisions
- Use a new InputMode::SelectingRecurrence to handle the dropdown state
- Display options as a vertical List widget with navigation via j/k keys
- Highlight selected option with the same styling as other UI elements
- Enter confirms selection, Esc cancels and returns to editing mode
- Render the List widget in place of the Recurrence Paragraph, using the same area
- Update hints to 'j/k: navigate, Enter: select, Esc: cancel'
- No external dependencies needed; leverage existing ratatui widgets

## Risks / Trade-offs
- Risk: Increased complexity in input handling logic
  - Mitigation: Isolate changes to recurrence field, reuse existing navigation patterns
- Trade-off: Slightly more key presses vs free-text speed
  - Justification: Better for users who don't remember syntax; free-text remains for power users if needed

## Migration Plan
No migration needed; this enhances existing UI without changing data formats or existing workflows.</content>
<parameter name="filePath">openspec/changes/add-recurrence-dropdown/design.md