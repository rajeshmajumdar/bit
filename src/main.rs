mod helpers;
use std::env;

// TODO: This is a test todo.
// FIXME: This is test fixme.

fn main() {
    let cwd = env::current_dir().unwrap();
    if helpers::is_git_repo(&cwd) {
        let files = helpers::get_all_files(&cwd);
        for file in files {
            let issues = helpers::get_issues(&file);
            for issue in issues {
                println!("{:?}", issue);
            }
        }
    }
}
