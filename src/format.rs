use std::fmt::Write;

use google_calendar::types::{Event, EventDateTime};

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
                let mut fmt = format!("- {} #meeting", event.summary);
                if let Some(date) = self.process_date(event.start.take(), event.end.take()) {
                    write!(fmt, "\n  - Date:: [[date:{}]]", date).unwrap();
                }
                if !event.attendees.is_empty() {
                    write!(
                        fmt,
                        "\n  - Attendees:: \n{}",
                        event
                            .attendees
                            .iter()
                            .map(|a| format!(
                                "    - [[{} #person]]",
                                eor(&a.display_name, &a.email)
                            ))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                    .unwrap();
                }
                fmt
            }
            Self::Raw => event.summary,
        })
    }

    fn process_date(
        self,
        start: Option<EventDateTime>,
        end: Option<EventDateTime>,
    ) -> Option<String> {
        Some(match self {
            Self::Tana => {
                let get_date = |date: Option<EventDateTime>| -> Option<String> {
                    let date = date?;
                    date.date_time
                        .map(|d| d.to_rfc3339())
                        .or(date.date.map(|d| d.to_string()))
                };

                let start = get_date(start)?;
                let end = get_date(end).unwrap_or("".to_string());
                format!("{}{}{}", start, if end.is_empty() { "" } else { "/" }, end)
            }
            Self::Raw => start.clone()?.date_time?.to_rfc3339(),
        })
    }
}

fn eor<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.is_empty() {
        s2
    } else {
        s1
    }
}
