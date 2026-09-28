use std::path::Path;
use std::time::Instant;

use notify_debouncer_full::DebouncedEvent;
use notify_debouncer_full::notify::Event;
use notify_debouncer_full::notify::EventKind;

#[must_use]
pub fn debounced_event(event_kind: EventKind, path: &Path) -> DebouncedEvent {
    DebouncedEvent::new(
        Event::new(event_kind).add_path(path.to_path_buf()),
        Instant::now(),
    )
}
