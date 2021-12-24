use std::fs::{DirBuilder, File};
use std::io::{self, Write, Read};
use std::process::Command;
use std::env;
use serde::{Serialize, Deserialize};
use std::path::{PathBuf, Path};
mod argparser;
pub mod macros;
mod help_indexes;

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    version: String,
    description: Option<String>,
}

use argparser::Opt;

const DEFAULT_MAIN: &str = "\
#include <iostream>

int main() {
    std::cout << \"Hello, World!\" << std::endl;
}";

fn main() -> io::Result<()> {
    let opt = argparser::parse_args();
    match opt {
        Opt::New { name } => {
            let mut dir_builder = DirBuilder::new();
            dir_builder.recursive(true);
            dir_builder.create(format!("{}/src", &name))?;
            dir_builder.create(format!("{}/out/debug", &name))?;
            dir_builder.create(format!("{}/out/release", &name))?;
            let mut main = File::create(format!("{}/src/main.cpp", &name))?;
            writeln!(main, "{}", DEFAULT_MAIN)?;
            main.sync_all()?;
            let mut project_yml = File::create(format!("{}/project.yml", &name))?;
            writeln!(project_yml, "name: {}", Path::new(&name).file_name().unwrap().to_str().unwrap())?;
            writeln!(project_yml, "version: 0.1.0")?;
            writeln!(project_yml, "description: ''")?;
            project_yml.sync_all()?;
        },
        Opt::Build { optimize } => {
            let mut projectfile = File::open("project.yml")?;
            let mut projectc = String::new();
            projectfile.read_to_string(&mut projectc)?;
            let project: Project = match serde_yaml::from_str(&projectc) {
                Ok(p) => p,
                Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
            };
            let cflags: Vec<String> = match env::var("CFLAGS") {
                Ok(a) => a.split_whitespace().map(String::from).collect(),
                Err(_) => vec![],
            };
            let mut command_build = Command::new("g++");
            let mut out_path = PathBuf::from("out/debug".to_string());
            if optimize {
                command_build.arg("-O3");
                out_path.pop();
                out_path.push("release");
            }
            out_path.push(project.name);
            command_build.arg("-o");
            command_build.arg(out_path.as_os_str());
            command_build.arg("src/main.cpp");
            command_build.args(&cflags);
            println!("running `g++{} -o {} src/main.cpp{}`",
                 if optimize {" -O3"} else {""},
                 out_path.as_path().display(),
                 if cflags.len() > 0 {
                     format!(" {}", cflags.join(" "))
                 } else {"".to_string()}
            );
            command_build.spawn()?.wait()?;
        },
        Opt::Run { optimize } => {
            let mut projectfile = File::open("project.yml")?;
            let mut projectc = String::new();
            projectfile.read_to_string(&mut projectc)?;
            let project: Project = match serde_yaml::from_str(&projectc) {
                Ok(p) => p,
                Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
            };
            let cflags: Vec<String> = match env::var("CFLAGS") {
                Ok(a) => a.split_whitespace().map(String::from).collect(),
                Err(_) => vec![],
            };
            let mut command_build = Command::new("g++");
            let mut out_path = PathBuf::from("out/debug".to_string());
            if optimize {
                command_build.arg("-O3");
                out_path.pop();
                out_path.push("release");
            }
            out_path.push(project.name);
            command_build.arg("-o");
            command_build.arg(out_path.as_os_str());
            command_build.arg("src/main.cpp");
            command_build.args(&cflags);
            println!("running `g++{} -o {} src/main.cpp{}`",
                 if optimize {" -O3"} else {""},
                 out_path.as_path().display(),
                 if cflags.len() > 0 {
                     format!(" {}", cflags.join(" "))
                 } else {"".to_string()}
            );
            command_build.spawn()?.wait()?;
            println!("running `{}`", out_path.as_path().display());
            // Run out/<debug level>/<program name>
            Command::new(format!("{}", out_path.as_path().display())).spawn()?.wait()?;
        },
        Opt::Help { exit_code, help_cat, .. } => help_indexes::print_help_for(help_cat, exit_code),
        Opt::Version => help_indexes::print_version(),
    }
    Ok(())
}

