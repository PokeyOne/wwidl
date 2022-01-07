# What Was I Doing Last
This project is meant to be a CLI tool for writing notes about what you were
doing last time you were working on a project/directory.

I realise after making this that most of the function of this app can be replaced
by simply writing better git commit messages, but I guess this can be used for
cases where there isnt' a git repo or if something more specific is needed.

[license](COPYING)

## Usage

The two main commands are `note` and `check`. `check` will show you
the latest note, and `note` will allow you to write a new note.

For details on how to use each command type
```bash
wwidl --help
```
or with no arguments will show the help text.

## Installation

To install the package, run the following command:
```bash
cargo install --path .
```
or use the install-script that will run the exact same command.
```bash
./install.sh
```
The cargo command should work on any system, but the install-script will
probably only work on Linux and macOS or similar.
