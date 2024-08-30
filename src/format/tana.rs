use super::{Event, Format};

pub struct Tana;

impl Format for Tana {
    fn format(event: Event) -> Option<String> {
        Some(event.title)
    }
}
