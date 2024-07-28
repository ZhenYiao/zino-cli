## Zino/cli Community Edition

### Installation
```shell
git clone https://github.com/ZhenYiao/zino-cli.git
cd zino-cli
cargo build --release
```

### Usage

```
=> zino-cli --help
Usage: zino-cli <COMMAND>

Commands:
  new    Create a new project
  serve  Serve the project
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
- zino-cli new
```
=> zino-cli new --help
Create a new project

Usage: zino-cli.exe new

Options:
  -h, --help  Print help
```
```
=> zino-cli new
     ______
     |__  / (_)  _ __     ___
       / /  | | | '_ \   / _ \
      / /_  | | | | | | | (_) |
     /____| |_| |_| |_|  \___/

2024-07-27T13:19:18.780901Z  INFO zino_cli::i18n: Set locale to en
Enter the project name: test
? Select the project type ›
❯ Actix App
  Axum App
  Ntex App
  dioxus-ssr
  dioxus-desktop
✔ Fetching crate 1/13
✔ Fetching crate 2/13
✔ Fetching crate 3/13
✔ Fetching crate 4/13
✔ Fetching crate 5/13
✔ Fetching crate 6/13
✔ Fetching crate 7/13
✔ Fetching crate 8/13
✔ Fetching crate 9/13
✔ Fetching crate 10/13
✔ Fetching crate 11/13
✔ Fetching crate 12/13
✔ Fetching crate 13/13


project test init successful

     cd test

     git init

     cargo run

If you have any questions or suggestions, please feel free to ask in the issue: https://github.com/ZhenYiao/zino-cli/issues
```
### Features
- [x] Create a new project(v0.1.0)
- [ ] Serve the project(v0.2.0)
- [ ] Build the project(v0.3.0)
- [ ] Deploy the project(v0.4.0)

### Suggestions
if you have any suggestions, please create an issue [here](https://github.com/ZhenYiao/zino-cli/issues/3)
