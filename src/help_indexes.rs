use crate::map;
use std::collections::HashMap;
use std::process::exit;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const TARGET: &str = env!("TARGET");

pub fn get_help_indexes() -> HashMap<String, String> {
    let m = map!{
        "".to_string() => String::from("\
USAGE: ultron <SUBCOMMAND> [OPTIONS]

SUBCOMMANDS:
    build b [OPTIONS]         build project with options
    run r [OPTIONS]           build and run project with options
    new <name>                new project named <name>
    help --help -h [ABOUT]    print help message and exit
    version --version -V      print version and exit")
    };
    m
}

pub fn print_help_for(s: String, exit_code: i32) -> ! {
    let indexes = get_help_indexes();
    println!("ultron version {} compiled for {}\n", VERSION, TARGET);

    match indexes.get(&s) {
        Some(s) => println!("{}", s),
        None => {
            println!("Couldn't find help for `{}`", &s);
            println!("Showing default help instead");
            // "" is always guaranteed to be there so unwrap is fine
            println!("{}", indexes.get(&String::from("")).unwrap());
        },
    }
    exit(exit_code);
}

pub fn print_version() -> ! {
    println!("ultron version {} compiled for {}", VERSION, TARGET);
    exit(0);
}

