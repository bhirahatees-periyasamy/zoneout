use std::fs;
use std::path::PathBuf;

pub fn domains_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join(".config").join("zoneout").join("domains.json")
}

pub fn load() -> Vec<String> {
    let path = domains_path();
    let data = match fs::read_to_string(&path) {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };
    serde_json::from_str(&data).unwrap_or_default()
}

pub fn add(domain: &str) -> std::io::Result<bool> {
    let mut domains = load();
    let domain = domain.trim_start_matches("www.").to_lowercase();
    if domains.contains(&domain) {
        return Ok(false); // already exists
    }
    domains.push(domain);
    save(&domains)?;
    Ok(true)
}

pub fn remove(domain: &str) -> std::io::Result<bool> {
    let mut domains = load();
    let domain = domain.trim_start_matches("www.").to_lowercase();
    let before = domains.len();
    domains.retain(|d| d != &domain);
    if domains.len() == before {
        return Ok(false); // not found
    }
    save(&domains)?;
    Ok(true)
}

pub fn remove_all() -> std::io::Result<()> {
    let path = domains_path();
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

fn save(domains: &[String]) -> std::io::Result<()> {
    let path = domains_path();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_string_pretty(domains)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    fs::write(path, json)
}
