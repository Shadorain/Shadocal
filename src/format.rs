use gcal::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Tana,
    Raw,
    // Custom(format_config)
}

impl Format {
    pub fn format(self, events: Vec<Event>) -> Option<String> {
        let header = match self {
            Self::Tana => "%%tana%%\n",
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
    pub fn event(self, event: Event) -> Option<String> {
        Some(match self {
            Self::Tana => {
                format!(
                    "{} #meeting \
                    Date:: [[{}/{}]]",
                    event.summary?, event.start?.date_time?, event.end?.date_time?
                )
            }
            Self::Raw => event.summary?,
        })
    }
}
