use std::env::args;
pub enum HelpError {
    NotEnoughArgs,
    SubcommandDoesntHaveEnoughArgs(String),
    UnknownSubcommand(String),
    None,
}

pub enum Opt {
    Build {
        optimize: bool,
    },
    Run {
        optimize: bool,
    },
    New {
        name: String,
    },
    Help {
        exit_code: i32,
        help_err: HelpError,
        help_cat: String,
    },
    Version,
}

pub fn parse_args() -> Opt {
    let args: Vec<String> = args().collect();

    let mut current: usize = 2;

    if args.len() < 2 {
        return Opt::Help {
            exit_code: 1,
            help_err: HelpError::NotEnoughArgs,
            help_cat: "".to_string(),
        }
    }

    match args.get(1).unwrap().as_str() {
        "-h" | "--help" | "help" => Opt::Help { exit_code: 0, help_err: HelpError::None, help_cat: "".to_string() },
        "new" | "n" => {
            if args.len() < 3 {
                Opt::Help {
                    exit_code: 1,
                    // Safe to unwrap because we made sure it will be there beforehand
                    help_err: HelpError::SubcommandDoesntHaveEnoughArgs(args.get(1).unwrap().to_string()),
                    help_cat: "new".to_string(),
                }
            } else {
                Opt::New {
                    name: args.get(2).unwrap().to_string(),
                }
            }
        },
        "build" | "b" => {
            let mut optimize = false;
            while current < args.len() {
                // Safe to unwrap because we made sure it will be there beforehand
                let cs = args.get(current).unwrap();
                match cs.as_str() {
                    "-r" | "--release" => optimize = true,
                    _ => unimplemented!(),
                }
                current += 1;
            }
            Opt::Build {
                optimize
            }
        },
        "run" | "r" => {
            let mut optimize = false;
            while current < args.len() {
                // Safe to unwrap because we made sure it will be there beforehand
                let cs = args.get(current).unwrap();
                match cs.as_str() {
                    "-r" | "--release" => optimize = true,
                    _ => unimplemented!(),
                }
                current += 1;
            }
            Opt::Run {
                optimize
            }
        },
        "version" | "--version" | "-V" => Opt::Version,
        subc => Opt::Help {
            exit_code: 1,
            help_err: HelpError::UnknownSubcommand(subc.to_string()),
            help_cat: "".to_string(),
        },
    }
}

