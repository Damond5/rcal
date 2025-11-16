# Spec Delta: Enhance End Date Input Assistance
# Modifies: openspec/specs/event-management/spec.md
# Date: 2025-11-16
# Purpose: Add real-time validation and auto-completion for end date input

## MODIFIED Requirements

### Requirement: End Date Format Handling
End date inputs MUST use DD/MM format with automatic year assumption and provide real-time validation, auto-completion suggestions, and visual feedback while maintaining the required DD/MM format.

#### Scenario: Date Input Parsing
Given DD/MM date,
When assuming current/next year,
Then date is correctly interpreted.

#### Scenario: Invalid Format Detection
Given end date input field,
When user enters invalid format (not DD/MM),
Then shows red error message below field.

#### Scenario: Valid Date Confirmation
Given end date input field,
When user enters valid DD/MM date,
Then clears any previous error messages.

#### Scenario: Suggestion Display
Given end date input field,
When user starts typing,
Then shows relevant date suggestions below field.

#### Scenario: Suggestion Selection
Given displayed suggestions,
When user presses Tab,
Then cycles through available suggestions.

#### Scenario: Relative Date Suggestions
Given event with start date,
When entering end date,
Then suggestions include "tomorrow", "next week", "end of month" relative to start date.