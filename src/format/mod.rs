mod tana;
pub use tana::*;

mod raw;
// pub use raw::*;

use super::{Event, EventStatus, EventType};

pub trait Format {
    fn newline() -> &'static str {
        "\n"
    }
    fn format(event: Event) -> Option<String>;
    fn format_list(events: Vec<Event>) -> Option<String> {
        Some(
            events
                .into_iter()
                .filter_map(|e| Self::format(e))
                .collect::<Vec<_>>()
                .join(Self::newline()),
        )
    }
}
