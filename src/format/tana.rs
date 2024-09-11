use super::{Event, EventType, Format};

pub struct Tana;

impl Format for Tana {
    // Yes I know how horribly this method is laid out XD
    fn format(event: Event) -> Option<String> {
        let mut output = format!(
            "- {} {}\n{}\n{}\n{}\n{}{}{}{}",
            event.title,
            supertag(&event),
            date(event.start, event.end),
            status(event.status),
            id(event.id),
            cal_id(&event.cal_id),
            description(event.description).unwrap_or_default(),
            attendees(event.attendees).unwrap_or_default(),
            location(event.location).unwrap_or_default(),
        );

        if let EventType::Meeting = event.event_type {
            output.push_str(&setting(event.cal_id));
            push(&mut output, link(event.link));
            push(&mut output, cal_link(event.cal_link));
        }

        Some(output)
    }
}

fn push(s: &mut String, val: Option<String>) {
    if let Some(v) = val {
        s.push_str(&v);
    }
}

fn supertag(event: &Event) -> &'static str {
    let title = event.title.to_ascii_lowercase();
    if title.contains("devoted") || title.contains("Church") {
        "#church-event"
    } else if title.contains("date") {
        "#date-night"
    } else if title.contains("birthday") {
        "#birthday"
    } else if title.contains("holiday") || event.cal_id.contains("holiday") {
        "#holiday"
    } else {
        match event.event_type {
            EventType::Meeting => "#meeting",
            EventType::Birthday => "#birthday",
            EventType::FocusTime => "#focus-time",
            EventType::OutOfOffice => "#time-off",
        }
    }
}

fn date(start: String, end: Option<String>) -> String {
    let end = end.unwrap_or("".to_string());
    format!(
        "  - Date:: [[date:{start}{}{end}]]",
        if end.is_empty() { "" } else { "/" },
    )
}
fn status(status: String) -> String {
    format!("  - [[Event status]]:: {status}")
}
fn attendees(attendees: Option<Vec<String>>) -> Option<String> {
    Some(format!(
        "\n  - Attendees:: \n{}",
        attendees?
            .iter()
            .map(|a| format!("    - [[{} #person]]", a))
            .collect::<Vec<_>>()
            .join("\n")
    ))
}
fn id(id: String) -> String {
    format!("  - [[Event ID]]:: {id}")
}
fn cal_id(cal_id: &str) -> String {
    format!("  - [[Calendar ID]]:: {}", cal_id)
}

fn description(description: Option<String>) -> Option<String> {
    Some(format!(
        "\n  - Description:: {}",
        description?.replace('\r', "").replace('\n', " ")
    ))
}
fn location(location: Option<String>) -> Option<String> {
    Some(format!("\n  - Location:: {}", location?))
}

fn link(link: Option<(&'static str, String)>) -> Option<String> {
    let (title, link) = link?;
    Some(format!("\n  - [[Meeting link]]:: [Join {title}]({link})"))
}

fn cal_link(link: Option<String>) -> Option<String> {
    Some(format!(
        "\n  - [[Calendar link]]:: [Go to Calendar]({})",
        link?
    ))
}

fn setting(cal_id: String) -> String {
    // Assumes work email wouldnt have a `gmail.com` extension.
    format!(
        "\n  - Setting:: {}",
        if cal_id.contains("gmail.com") {
            "[[🏠 Home/Personal]]"
        } else {
            "[[💼 Work]]"
        }
    )
}
