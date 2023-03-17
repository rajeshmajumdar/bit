use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader};
use std::io::prelude::*;


use reqwest::header::AUTHORIZATION;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Issue {
    title: String,
    body: String,
    labels: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Response {
    number: u64,
    title: String,
    body: String,
    state: String,
}

#[derive(Serialize, Deserialize)]
struct CloseIssue {
    state: String
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
                    if line.contains(issue) {
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

fn get_label(comment: &str) -> Option<&str> {
    let special_comments = vec!["TODO", "FIXME", "BUG", "NOTE", "HACK", "OPTIMIZATION", "IDEA"];
    let mut res = None;
    for comms in special_comments {
        if comment.contains(comms) {
            res = match comms {
                "TODO" => Some("enhancement"),
                "FIXME" => Some("bug"),
                "BUG" => Some("bug"),
                "NOTE" => Some("documentation"),
                "HACK" => Some("enhancement"),
                "OPTIMIZATION" => Some("enhancement"),
                "IDEA" => Some("wontfix"),
                _ => None
            }
        }
    }
    res
}

// TODO: Add more special comments.
pub fn create_issue(mut comment: String) -> Result<(), Box<dyn std::error::Error>> {
    let token = get_git_creds().unwrap_or_default();
    let repo_data = get_repo_info().unwrap_or_default();
    let (owner, repo) = repo_data;
    let url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);

    if !already_issued(&comment) {
        let label = match get_label(&comment) {
            Some(val) => val,
            None => {
                return Err("Label not found in special comments.".to_string().into());
            }
        };
        if let Some(index) = comment.find(":") {
            let (_, comm) = comment.split_at(index + 1);
            let title = comm.trim();
            let body = comm.trim();


            let payload = Issue {
                title: title.to_string(),
                body: body.to_string(),
                labels: vec![label.to_string()],
            };
            let issue = serde_json::to_string(&payload).unwrap();

            let client = reqwest::blocking::Client::new();
            let res = client.post(&url)
                .header(AUTHORIZATION, format!("Bearer {}", token))
                .header("User-Agent", "flagit")
                .body(issue)
                .send()
                .unwrap();
            let resp_body = res.text().unwrap();
            let issue: Response = serde_json::from_str(&resp_body).unwrap();
            let issue_number = issue.number;
            comment = comment + " - " + &issue_number.to_string();
            write_issue(&comment)?;
        }

        Ok(())
    } else {
        Ok(())
    }
}

fn get_comment(issue: &str) -> Option<String> {
    if let Some(colon_index) = issue.find(':') {
        if let Some(dash_index) = issue.rfind('-') {
            if colon_index < dash_index {
                let comment = issue[colon_index..dash_index].trim();
                return Some(comment.to_string());
            }
        }
    }
    None
}

fn remove_line_from_file(line: &str) -> std::io::Result<()> {
    let file_path = "flagit.lock";
    let temp = file_path.to_owned() + ".temp";
    let input_file = std::fs::File::open(file_path).unwrap();
    let temp_file = std::fs::File::create(&temp)?;

    let mut writer = std::io::BufWriter::new(temp_file);

    let reader = BufReader::new(input_file);
    for file_line in reader.lines() {
        let current_line = file_line?;
        if current_line != line {
            writeln!(writer, "{}", current_line)?;
        }
    }

    writer.flush()?;
    std::fs::remove_file(file_path)?;
    std::fs::rename(&temp, file_path)?;
    Ok(())
}

pub fn close_issue_if_completed(issue: &str) {
    let file = std::fs::File::open("flagit.lock").unwrap();
    let reader = BufReader::new(file);
    let mut not_completed: bool = false;
    for line in reader.lines() {
        if let Ok(line) = line {
            let comment = match get_comment(&line) {
                Some(val) => val,
                None => continue,
            };
            if issue.contains(&comment) {
                not_completed = true;
            }
            if !not_completed {
                unimplemented!();
                /*
                let issue_number = line.rsplitn(2, '-').next().unwrap().trim();
                let repo_data = get_repo_info().unwrap_or_default();
                let (owner, repo) = repo_data;
                let token = get_git_creds().unwrap_or_default();
                let url = format!("https://api.github.com/repos/{}/{}/issues/{}", owner, repo, issue_number);
                let payload = CloseIssue {
                    state: "closed".to_string()
                };
                let body = serde_json::to_string(&payload).unwrap();
                let client = reqwest::blocking::Client::new();
                remove_line_from_file(&line).unwrap();
                let _res = client.patch(&url)
                    .header(reqwest::header::USER_AGENT, "flagit")
                    .header(reqwest::header::AUTHORIZATION, token)
                    .body(body)
                    .send()
                    .unwrap();
                */
            }
        }
    }
}
