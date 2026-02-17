use jiff::civil::DateTime;
use til::error::{TimelineError, TimelineResult};

pub fn parse_datetime(s: &str) -> TimelineResult<DateTime> {
    if let Ok(dt) = s.parse::<DateTime>() {
        return Ok(dt);
    }
    if let Ok(d) = s.parse::<jiff::civil::Date>() {
        return Ok(d.at(0, 0, 0, 0));
    }
    if s.len() == 8 && s.bytes().all(|b| b.is_ascii_digit()) {
        if let Some(d) = parse_compact_date(s) {
            return Ok(d.at(0, 0, 0, 0));
        }
    }
    if let Some(d) = parse_dashed(s) {
        return Ok(d.at(0, 0, 0, 0));
    }
    if let Some(d) = parse_named_month(s) {
        return Ok(d.at(0, 0, 0, 0));
    }
    if let Ok(year) = s.parse::<i16>() {
        if let Ok(d) = jiff::civil::Date::new(year, 1, 1) {
            return Ok(d.at(0, 0, 0, 0));
        }
    }
    Err(TimelineError::DateTimeParse(s.to_string()))
}

pub fn format_datetime(dt: &DateTime) -> String {
    if dt.hour() == 0 && dt.minute() == 0 && dt.second() == 0 {
        dt.date().to_string()
    } else {
        dt.to_string()
    }
}

fn parse_compact_date(s: &str) -> Option<jiff::civil::Date> {
    let year: i16 = s[0..4].parse().ok()?;
    let month: i8 = s[4..6].parse().ok()?;
    let day: i8 = s[6..8].parse().ok()?;
    jiff::civil::Date::new(year, month, day).ok()
}

fn parse_dashed(s: &str) -> Option<jiff::civil::Date> {
    let parts: Vec<&str> = s.split('-').collect();
    match parts.len() {
        2 => {
            let year: i16 = parts[0].parse().ok()?;
            let month: i8 = parts[1].parse().ok()?;
            jiff::civil::Date::new(year, month, 1).ok()
        }
        3 => {
            let year: i16 = parts[0].parse().ok()?;
            let month: i8 = parts[1].parse().ok()?;
            let day: i8 = parts[2].parse().ok()?;
            jiff::civil::Date::new(year, month, day).ok()
        }
        _ => None,
    }
}

fn parse_named_month(s: &str) -> Option<jiff::civil::Date> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    match parts.len() {
        2 => {
            let month = month_from_name(parts[0])?;
            let year: i16 = parts[1].parse().ok()?;
            jiff::civil::Date::new(year, month, 1).ok()
        }
        3 => {
            let month = month_from_name(parts[0])?;
            let day: i8 = parts[1].trim_end_matches(',').parse().ok()?;
            let year: i16 = parts[2].parse().ok()?;
            jiff::civil::Date::new(year, month, day).ok()
        }
        _ => None,
    }
}

fn month_from_name(s: &str) -> Option<i8> {
    match s.to_lowercase().as_str() {
        "jan" | "january" => Some(1),
        "feb" | "february" => Some(2),
        "mar" | "march" => Some(3),
        "apr" | "april" => Some(4),
        "may" => Some(5),
        "jun" | "june" => Some(6),
        "jul" | "july" => Some(7),
        "aug" | "august" => Some(8),
        "sep" | "september" => Some(9),
        "oct" | "october" => Some(10),
        "nov" | "november" => Some(11),
        "dec" | "december" => Some(12),
        _ => None,
    }
}
