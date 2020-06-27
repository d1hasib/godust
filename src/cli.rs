use std::{
    env, fs,
    io::{self, prelude::*},
    path,
};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "godust",
    about = "Godot project generation tool for the Rustaceans."
)]
enum Opt {
    /// Creates a new project
    New {
        /// Project name
        name: String,
    },
}

pub struct Project {
    name: String,
    path: path::PathBuf,
}

impl Project {
    pub fn new() -> Self {
        let opt = Opt::from_args();
        let mut proj_name = String::new();
        let proj_path;

        #[allow(irrefutable_let_patterns)]
        if let Opt::New { name } = opt {
            proj_name = name;
        }

        proj_path = env::current_dir()
            .expect("can't get the current directory")
            .join(name_formatter(&proj_name));

        Project {
            name: proj_name,
            path: proj_path,
        }
    }

    pub fn build_structure(&self) -> io::Result<()> {
        // Creates directories
        fs::create_dir_all(&self.path.join("src"))?;

        // Creates project.godot file and insert the text accordingly
        let mut godot_file = fs::File::create(&self.path.join("project.godot"))?;
        godot_file.write_all(get_godot_text(&self.name).as_bytes())?;

        // Creates Cargo.toml file and insert the text accordingly
        let mut cargo_file = fs::File::create(&self.path.join("Cargo.toml"))?;
        cargo_file.write_all(get_cargo_text(&self.name).as_bytes())?;

        // Creates lib.rs file and insert the text accordingly
        let mut rustlib_file = fs::File::create(&self.path.join("src/lib.rs"))?;
        rustlib_file.write_all(get_rustlib_text().as_bytes())?;

        // Creates project.gdnlib file and insert the text accordingly
        let mut gdnlib_file = fs::File::create(
            &self
                .path
                .join(format!("{}.gdnlib", name_formatter(&self.name))),
        )?;
        gdnlib_file.write_all(get_gdnlib_text(&self.name).as_bytes())?;

        // Prints out name, path and the structure of the project
        println!(
            "{}",
            get_output_text(
                &self.name,
                match self.path.to_str() {
                    Some(p) => p,
                    None => panic!("can't convert path to string slice"),
                }
            )
        );
        println!();

        Ok(())
    }
}

fn name_formatter(name: &str) -> String {
    name.to_lowercase().replace(" ", "_")
}

fn get_godot_text(name: &str) -> String {
    format!(
        "\
[application]
config/name=\"{}\"",
        name
    )
}

fn get_cargo_text(name: &str) -> String {
    format!(
        "\
[package]
name = \"{}\"
version = \"0.1.0\"
edition = \"2018\"

[lib]
crate-type = [\"cdylib\"]

[dependencies]
gdnative = \"0.8\"",
        name_formatter(name)
    )
}

fn get_rustlib_text() -> String {
    format!(
        "\
use gdnative::*;

#[derive(NativeClass)]
#[inherit(Node)]
struct HelloWorld;

#[methods]
impl HelloWorld {{
    fn _init(_owner: Node) -> Self {{
        HelloWorld
    }}

    #[export]
    fn _ready(&self, _owner: Node) {{
        godot_print!(\"Hello, World!\")
    }}
}}

fn init(handle: init::InitHandle) {{
    handle.add_class::<HelloWorld>();
}}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();"
    )
}

fn get_gdnlib_text(name: &str) -> String {
    format!(
        "\
[general]

singleton=false
load_once=true
symbol_prefix=\"godot_\"
reloadable=true

[entry]

X11.64=\"res://target/debug/lib{name}.so\"
OSX.64=\"res://target/debug/lib{name}.dylib\"
Windows.64=\"res://target/debug/{name}.dll\"",
        name = name_formatter(name),
    )
}

fn get_output_text(name: &str, path: &str) -> String {
    format!(
        "\
The project has been generated.

Name: \"{name}\"
Path: {path:?}

Structure:  {name}
            ├── src
            │   └── lib.rs
            ├── Cargo.toml
            ├── {name_lc}.gdnlib
            └── project.godot",
        name = name,
        name_lc = name_formatter(name),
        path = path,
    )
}
