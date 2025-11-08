# Remove H/L Navigation

## Why
The H/L navigation keys for paging through three-month periods are redundant with the existing h/j/k/l navigation that already shifts the view when moving beyond boundaries. This creates confusion and unnecessary complexity in the navigation system. PageUp/PageDown keys are also mapped to the same paging functionality and should be removed for consistency.

## What Changes
- Remove H/L and PageUp/PageDown key handling from event processing
- Remove H/L references from UI hints
- Remove H/L paging requirements from ui-rendering spec
- Simplify navigation to rely solely on h/j/k/l with automatic view shifting

## Impact
- Affected specs: ui-rendering
- Affected code: event_handling.rs, ui.rs