# ui-rendering Specification Delta

## REMOVED Requirements

### Requirement: Sync Status Keybind Instruction
The sync popup MUST NOT display "s: Check status" instruction.

#### Scenario: Clean Sync Instructions
Given sync popup is displayed,
When user views available actions,
Then only "f: Pull from remote" and "p: Push to remote" are shown.

Note: Existing requirements for displaying "f: Pull from remote" and "p: Push to remote" instructions remain unchanged.</content>
<parameter name="filePath">openspec/changes/2025-11-08-remove-sync-status-keybind-instruction/specs/ui-rendering/spec.md