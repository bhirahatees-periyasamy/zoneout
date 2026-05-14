use std::time::Duration;

pub fn parse_hms(s: &str) -> Result<u64, String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 3 {
        return Err(format!("Invalid time format '{}': expected HH:MM:SS", s));
    }
    let h: u64 = parts[0].parse().map_err(|_| format!("Invalid hours in '{}'", s))?;
    let m: u64 = parts[1].parse().map_err(|_| format!("Invalid minutes in '{}'", s))?;
    let sec: u64 = parts[2].parse().map_err(|_| format!("Invalid seconds in '{}'", s))?;
    if m >= 60 || sec >= 60 {
        return Err(format!("Minutes and seconds must be < 60 in '{}'", s));
    }
    Ok(h * 3600 + m * 60 + sec)
}

pub fn duration_from_args(
    time: Option<&str>,
    hours: Option<u64>,
    minutes: Option<u64>,
) -> Result<Duration, String> {
    let total_secs = if let Some(t) = time {
        parse_hms(t)?
    } else {
        hours.unwrap_or(0) * 3600 + minutes.unwrap_or(0) * 60
    };
    if total_secs == 0 {
        return Err("Duration must be greater than zero".to_string());
    }
    Ok(Duration::from_secs(total_secs))
}

pub fn fmt_duration_secs(total: u64) -> String {
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;
    if h > 0 {
        format!("{}h {}m {}s", h, m, s)
    } else if m > 0 {
        format!("{}m {}s", m, s)
    } else {
        format!("{}s", s)
    }
}
