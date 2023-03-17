# BIT - A Better Issue Tracker for git.
A CLI tool to manage your TODOs and FIXMEs. It pulls out every TODOs and other such special comments in your codebase and create an issue on github with respective labels..

## Quick setup
Bit relies on `.git` folder to retrieve the repository and the username.

```console
$ git init
$ git remote add origin <your-repository-url>
```

Bit uses Github API to open issues, so for that we will need API key from Github.
You can google the same, on how to get one.

```console
$ echo <YOUR_API_TOKEN> > ~/.bitrc
```

## Quick start
Just execute it in your project folder, no need for any arguments.
```console
$ cd <YOUR-PROJECT-FOLDER>
$ bit
```

### Special comments
- `TODO`
- `FIXME`
- `BUG`
- `OPTIMIZATION`
- `HACK`
- `IDEA`

### Future features
- [ ] Support for different OSes (this shouldn't be tough, still need testing)
- [ ] Better code management
- [ ] Support for different git platforms (currently only supports github).
- [ ] Better track the issues within the editor.
- [ ] Close issues by simply removing the comments.
- [ ] Handle when executed for the first time.

### Contribution
Feel free to submit a PR. The project itself is very simple, currently all the git actions are handled in 'git.rs' and other utilities are handled in `helpers.rs`.
