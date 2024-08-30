mod tana;
pub use tana::*;

mod raw;
pub use raw::*;

use super::Event;

pub trait Format {
    fn format(event: Event) -> Option<String>;
    fn newline() -> &'static str {
        "\n"
    }
}
