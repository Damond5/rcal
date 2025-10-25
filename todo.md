# Todo
- When on the events popup, navigating up and down at the bounds should cycle to the other bound.
- Sandbox tests, so they dont mess with the currently stored events

## Potential Enhancements
- Calendar import/export features
- Add a textfield for notification minutes when creating/editing events
- Restructure project into a Cargo workspace with separate crates (`rcal-core` for shared models/state/persistence, `rcal-tui` for UI/event handling, `rcal-daemon` for notifications) to enable parallel development by multiple teams without merge conflicts. This includes internal modularization (e.g., split `app.rs` into `models.rs`, `state.rs`, `logic.rs`; add `calendar.rs` for date logic) and workspace-level shared elements (tests, docs). Benefits: Isolation, reduced conflicts, scalability; drawbacks: More complex setup.

