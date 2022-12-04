# Kakisute [![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)

Kakisute(scribbled snippets) management tool written in Rust.

# Install
## Using Homebrew

```sh
brew tap kazu914/kakisute/kakisute
brew install kazu914/kakisute
```

## Download binary from release page
Binaries are available in [release page](https://github.com/kazu914/kakisute/releases)


# Usage
## General
```sh
> kakisute help

USAGE:
    kakisute [OPTIONS] <SUBCOMMAND>

OPTIONS:
        --data_dir <DATA_DIR>    <Optional> Specify the directory to store kakisute files
    -h, --help                   Print help information
    -V, --version                Print version information

SUBCOMMANDS:
    new           Create new kakisute
    list          Print kakisute files
    edit          Edit existing kakiste
    show          Show existing kakisute
    inspect       Inspect existing kakisute
    delete        Delete existing kakisute
    interact      Start TUI mode
    completion    Generate completion script
    help          Print this message or the help of the given subcommand(s)
```

## Create new kakisute
```sh
> kakisute new --help

kakisute-new
Create new kakisute

USAGE:
    kakisute new [KAKISUTE_NAME]

ARGS:
    <KAKISUTE_NAME>    <Optional> Specify kakisute name

OPTIONS:
    -h, --help    Print help information

```

## Other subcommands
You can get help by
```sh
kakisute <SUBCOMMAND> --help
```

# Interact Mode
You can enter TUI mode by
```sh
kakisute interact
```

# Demo
![demo](https://user-images.githubusercontent.com/43592915/205480049-d26a97da-f634-4bb9-a887-be79536c3118.gif)

## Interact mode

<img width="904" alt="image" src="https://user-images.githubusercontent.com/43592915/205480138-47c5f543-2bd6-452a-a38d-b261aa54d5a7.png">

<img width="948" alt="image" src="https://user-images.githubusercontent.com/43592915/205480219-e628111d-9180-46e8-8f9b-cf2dbd83f074.png">

<img width="948" alt="image" src="https://user-images.githubusercontent.com/43592915/205480246-a81d483a-dd9d-4a36-b7c7-e5df693d16a7.png">
