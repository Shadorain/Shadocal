use super::{Event, Format};

pub struct Raw;

impl Format for Raw {
    fn format(event: Event) -> Option<String> {
        Some(event.title)
    }
}
