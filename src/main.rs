mod helpers;
mod git;
use std::env;

// TODO: Better error handling.
// TODO: Support for different OS & platforms.
// TODO: Usage screen.
// TODO: Ability to change to different git client.

fn usage() {
    println!("Usage: bit");
    println!("");
    println!("Find all TODO and FIXME comments in your project and create issues on Git.");
    println!("");
    println!("Optional arguments:");
    println!("    --client=[GIT_CLIENT]    Change your git client, default: Github.");
    println!("                             Please check documentation for supported clients.");
    println!("");
}

fn main() {
    let cwd = env::current_dir().unwrap();
    if git::is_git_repo(&cwd) {
        let files = helpers::get_all_files(&cwd);
        for file in files {
            let issues = helpers::get_issues(&file);
            //git::close_issue_if_completed(&issues);
            for issue in issues {
                git::create_issue(issue.clone()).unwrap();
            }
        }
    } else {
        println!("ERROR: Not a git repository.");
        usage();
    }
}
