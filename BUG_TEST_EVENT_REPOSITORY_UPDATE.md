# Bug Report: `test_event_repository_update` Test Failure

## 1. Summary

A pre-existing bug exists in the `InMemoryRepository::save` method that causes the `test_event_repository_update` test to fail. The implementation uses a composite key of `title + start_date` to locate existing events for updates, but this approach fails when the title field is modified before saving. The bug results in duplicate events being inserted instead of updating the existing event.

**Note:** This bug is unrelated to the notifications bug fix that was being investigated. It is a separate, pre-existing issue in the event repository implementation.

---

## 2. Issue Details

| Field | Value |
|-------|-------|
| **Test Name** | `test_event_repository_update` |
| **Test Location** | `rcal-lib/src/storage/traits.rs` |
| **Failure Message** | `assertion left == right failed: left: 2, right: 1` |
| **Observed Behavior** | Two events found after an update operation was attempted |
| **Expected Behavior** | One event should exist after updating its title |

---

## 3. Root Cause

The `save` method in `InMemoryRepository` (lines 96-98) constructs a lookup key using `title + start_date` to determine whether to insert a new event or update an existing one:

```rust
let key = format!("{}:{}", event.title, event.start_date);
```

This approach is fundamentally flawed because:

1. **The `title` field is mutable** — users can change an event's title after creation
2. **The key changes when the title changes** — if the title is updated, the lookup key changes
3. **The old event becomes unfindable** — the repository looks for the new key but the old key still stores the original event

### Scenario Breakdown

1. Initial save with title "Test Event":
   - Key: `"Test Event:2024-01-15"`
   - Event stored at this key

2. Test modifies title to "Updated Title" and calls save again:
   - New key: `"Updated Title:2024-01-15"`
   - Repository searches for this key — not found
   - Repository inserts a **new** event instead of updating
   - Result: Two events exist, original still at old key

---

## 4. Expected vs Actual Behavior

### Expected Behavior

| Operation | Result |
|-----------|--------|
| Create event with title "Test Event" | One event stored with ID `X` |
| Update title to "Updated Title" and save | Same event with ID `X` has updated title |
| Final state | **One event** with ID `X` and title "Updated Title" |

### Actual Behavior

| Operation | Result |
|-----------|--------|
| Create event with title "Test Event" | One event stored with ID `X` |
| Update title to "Updated Title" and save | New event with ID `Y` inserted at new key |
| Final state | **Two events**: original at `"Test Event:date"` + new at `"Updated Title:date"` |

---

## 5. Suggested Fix

The `save` method should use `id` as the lookup key for updates, not `title + start_date`. The `id` is a stable, immutable identifier assigned at event creation.

### Recommended Implementation

Replace the current key construction:

```rust
let key = format!("{}:{}", event.title, event.start_date);
```

With an ID-based lookup:

```rust
let key = event.id.to_string();
```

This ensures that:
- Updates always find the correct event regardless of field modifications
- The `id` remains constant throughout the event's lifecycle
- The repository correctly updates existing events rather than creating duplicates

### Alternative Considerations

If the repository interface requires finding events by human-readable keys for other use cases, consider:

1. Adding a separate `update_by_id` method that uses `id` for updates
2. Maintaining a secondary index that maps `id` → lookup key
3. First searching by `id`, then falling back to `title + start_date` for legacy compatibility

However, the simplest and most robust solution is to use `id` as the primary update key, as it guarantees stability and prevents the duplicate insertion bug.
