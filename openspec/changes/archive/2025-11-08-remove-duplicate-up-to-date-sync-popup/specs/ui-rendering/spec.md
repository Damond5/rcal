## ADDED Requirements

### Requirement: Sync Status Message Display
The sync popup MUST NOT display redundant "Up to date" messages when the sync status is up to date. Only the status line "Status: Up to date" should be shown.

#### Scenario: No Duplicate Up to Date Message
Given the sync status is UpToDate,
When displaying the sync popup,
Then only "Status: Up to date" is shown without additional "Up to date" message.