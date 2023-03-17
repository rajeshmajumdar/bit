mod helpers;
mod git;
use std::env;


fn main() {
    let cwd = env::current_dir().unwrap();
    if git::is_git_repo(&cwd) {
        let files = helpers::get_all_files(&cwd);
        for file in files {
            let issues = helpers::get_issues(&file);
            for issue in issues {
                git::create_issue(issue).unwrap();
            }
        }
    }
}
