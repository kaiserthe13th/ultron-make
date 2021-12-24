use crate::map;
use std::collections::HashMap;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_help_indexes() -> HashMap<String, String> {
    let m = map!{
        "".to_string() => format!("\
ultron version {} compiled for

USAGE: ultron <SUBCOMMAND> [OPTIONS]

SUBCOMMANDS:
    help --help -h       print help message and exit
    build b [OPTIONS]    build project with options
    run r [OPTIONS]      build and run project with options
    new <name>           new project named <name>
", VERSION)
    };
    m
}
