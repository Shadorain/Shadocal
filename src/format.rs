use std::fmt::Write;

use gcal::{Event, EventCalendarDate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Tana,
    Raw,
    // Custom(format_config)
}

impl Format {
    pub fn format(self, events: Vec<Event>) -> Option<String> {
        let header = match self {
            Self::Tana => "",
            Self::Raw => "",
        };
        Some(format!(
            "{header}{}",
            events
                .into_iter()
                .flat_map(|e| self.event(e))
                .collect::<Vec<String>>()
                .join("\n")
        ))
    }
    pub fn event(self, mut event: Event) -> Option<String> {
        Some(match self {
            Self::Tana => {
                let mut fmt = format!("- {} #meeting", event.summary?);
                if let Some(date) = self.process_date(event.start.take(), event.end.take()) {
                    write!(fmt, "\n  - Date:: [[date:{}]]", date).unwrap();
                }
                if let Some(attendees) = event.attendees {
                    write!(
                        fmt,
                        "\n  - Attendees:: \n{}",
                        attendees
                            .iter()
                            .map(|a| format!(
                                "    - [[{} #person]]",
                                a.display_name.as_deref().unwrap_or(a.email.as_ref())
                            ))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                    .unwrap();
                }
                fmt
            }
            Self::Raw => event.summary?,
        })
    }

    fn process_date(
        self,
        start: Option<EventCalendarDate>,
        end: Option<EventCalendarDate>,
    ) -> Option<String> {
        Some(match self {
            Self::Tana => {
                let get_date = |date: Option<EventCalendarDate>| -> Option<String> {
                    let date = date?;
                    date.date_time.or(date.date)
                };

                let start = get_date(start)?;
                let end = get_date(end).unwrap_or("".to_string());
                format!("{}{}{}", start, if end.is_empty() { "" } else { "/" }, end)
            }
            Self::Raw => start.clone()?.date_time?,
        })
    }
}
