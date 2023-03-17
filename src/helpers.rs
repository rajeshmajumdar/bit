use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader};
use std::io::prelude::*;


// FIXME: Currently only using .gitignore to ignore filetypes.
fn is_ignored(path: &std::path::PathBuf) -> bool {
    if let Ok(contents) = fs::read_to_string(".gitignore") {
        for line in contents.lines() {
            let line = line.trim();
            if line.starts_with("#") || line.is_empty() {
                continue;
            }
            let pattern = line.replace("**", ".*").replace("*", "[^/]*");
            if path.to_string_lossy().contains(&pattern) {
                return true;
            } else if path.to_string_lossy().contains(".git") {
                return true;
            }
        }
    }
    false
}

pub fn get_all_files(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut result = Vec::new();
    let mut files: Vec<std::path::PathBuf> = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            result.push(path);
        } else {
            result.extend(get_all_files(&path));
        }
    }
    for file in result {
        if !is_ignored(&file) {
            files.push(file);
        }
    }
    files
}

pub fn get_issues(path: &std::path::Path) -> Vec<String> {
    let mut issues = Vec::new();
    let file = fs::File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let special_comments = vec!["TODO", "FIXME", "BUG", "NOTE", "HACK", "OPTIMIZATION", "IDEA"];

    for line in reader.lines() {
        let line = line.unwrap();
        for comment in special_comments.iter() {
            if line.contains(comment) {
                issues.push(line.to_owned());
            }
        }
    }
    issues
}
