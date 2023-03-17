# Flagit
A CLI tool to manage your TODOs and FIXMEs.

## Quick setup
Flagit relies on `.git` folder to retrieve the repository and the username.

```console
$ git init
$ git remote add origin <your-repository-url>
```

Flagit uses Github API to open issues, so for that we will need API key from Github.
You can google the same, on how to get one.

```console
$ echo <YOUR_API_KEY> > ~/.flagitrc
```

## Quick start
Just execute it in your project folder, no need for any arguments.
```console
$ cd <YOUR-PROJECT-FOLDER>
$ flagit
```
