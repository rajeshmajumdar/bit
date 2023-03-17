use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader};
use std::io::prelude::*;


use reqwest::header::AUTHORIZATION;
use serde::{Serialize, Deserialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize)]
struct Issue {
    title: String,
    body: String,
    labels: Vec<String>,
}

fn write_issue(issue: &str) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open("flagit.lock")
        .unwrap();
    let issue = format!("{}\n", issue);
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(issue.as_bytes())?;

    Ok(())
}

fn already_issued(issue: &str) -> bool {
    match std::fs::File::open("flagit.lock") {
        Ok(file) => {
            let reader = BufReader::new(file);
            for lines in reader.lines() {
                if let Ok(line) = lines {
                    if line == issue {
                        return true;
                    }
                }
            }
            false
        },
        Err(_) => {
            false
        }
    }
}

pub fn is_git_repo(dir: &std::path::Path) -> bool {
    let path = dir.join(".git");
    std::fs::metadata(path).is_ok()
}

fn get_repo_info() -> Option<(String, String)> {
    let config_file = std::fs::File::open(".git/config").ok()?;
    let reader = BufReader::new(config_file);
    let mut lines = reader.lines().filter_map(|line| line.ok());

    while let Some(line) = lines.next() {
        if line.trim() == "[remote \"origin\"]" {
            while let Some(line) = lines.next() {
                if let Some(url) = line.trim().strip_prefix("url = ") {
                    let url = url.strip_suffix(".git").unwrap_or(url).to_string();
                    let parts: Vec<&str> = url.split('/').collect();
                    if let (Some(owner), Some(repo)) = (parts.get(parts.len() - 2), parts.last()) {
                        return Some((owner.to_string(), repo.to_string()));
                    }
                }
            }
        }
    }
    None
}

// TODO: Support for windows
fn get_git_creds() -> Option<String> {
    let home = match std::env::var_os("HOME") {
        Some(path) => path,
        None => panic!("HOME environment not set."),
    };
    let cred_path = home.to_str().unwrap_or_default().to_owned() + "/.flagitrc";
    let file = std::fs::File::open(cred_path).ok()?;
    let mut lines = BufReader::new(file).lines().filter_map(|line| line.ok());

    while let Some(line) = lines.next() {
        if line.trim() == "[creds]" {
            while let Some(line) = lines.next() {
                if let Some(token) = line.trim().strip_prefix("token = ") {
                    return Some(token.to_owned());
                }
            }
        }
    }
    None
}

// TODO: Add more special comments.
pub fn create_issue(comment: String) -> Result<(), Box<dyn std::error::Error>> {
    let token = get_git_creds().unwrap_or_default();
    let repo_data = get_repo_info().unwrap_or_default();
    let (owner, repo) = repo_data;
    let url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);

    if !already_issued(&comment) {
        let label = if comment.contains("TODO") { "enhancement" } else { "bug" };
        write_issue(&comment)?;
        if let Some(index) = comment.find(":") {
            let (_, comment) = comment.split_at(index + 1);
            let title = comment.trim();
            let body = comment.trim();


            let payload = Issue {
                title: title.to_string(),
                body: body.to_string(),
                labels: vec![label.to_string()],
            };
            let issue = to_string(&payload).unwrap();

            let client = reqwest::blocking::Client::new();
            let res = client.post(&url)
                .header(AUTHORIZATION, format!("Bearer {}", token))
                .header("User-Agent", "flagit")
                .body(issue)
                .send()
                .unwrap();
        }

        Ok(())
    } else {
        Ok(())
    }
}
