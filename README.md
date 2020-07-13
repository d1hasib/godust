# godust

A tool for structuring a new [Godot](https://godotengine.org/)
project flexible for using [Rust](https://www.rust-lang.org/)
programming language.

## Structure

```bash
Project
├── src
│   └── lib.rs
├── Cargo.toml
├── project.gdnlib
└── project.godot
```

## Installation

Run this command in the terminal-
```bash
cargo install --git https://github.com/d1hasib/godust
```
If you haven't do it already, add the following line in your
`.bashrc` or `.zshrc` file.
```
export PATH=$PATH:$HOME/.cargo/bin
```
This will help `shell` to find `godust`
## Usage
```bash
godust new {name}
```
**NB:** Godot will show a warning message after opening the
project for the first time-

> The following project settings file does not specify the
> version of Godot through which it was created.

`Ignore` the message and press the button <kbd>OK</kbd>
