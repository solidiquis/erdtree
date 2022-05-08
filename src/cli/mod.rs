#![allow(dead_code)]

use std::cmp::PartialEq;
use std::fs;
use std::process;

const HELP: &'static str = r#"
Usage:
    erdtree [directory] [options]

ARGUMENTS:
    directory     Directory to traverse. Defaults to current working directory.

OPTIONS:
    -l            Unsigned integer indicating many nested directory levels to display. Defaults to all.
    -p            Comma-separated list of prefixes. Directories containing any of
                  these prefixes will not be traversed. Their memory size will also be ignored.
    -s [asc|desc] Sort tree by memory-size. 
    -h            Displays help prompt.
"#;

/// Struct over valid command line options.
pub struct CommandLineArgs {
    pub directory: Option<String>,
    pub depth: Option<u64>,
    pub prefixes: Option<String>,
    pub sort_type: Option<String>
}

impl Default for CommandLineArgs {
    fn default() -> Self {
        CommandLineArgs { directory: None, depth: None, prefixes: None, sort_type: None }
    }
}

impl CommandLineArgs {
    pub fn set_directory(&mut self, directory: String) {
        self.directory = Some(directory);
    }

    pub fn set_depth(&mut self, depth: u64) {
        self.depth = Some(depth);
    }

    pub fn set_prefixes(&mut self, prefixes: String) {
        self.prefixes = Some(prefixes);
    }

    pub fn set_sort_type(&mut self, sort_type: String) {
        self.sort_type = Some(sort_type);
    }
}

/// Enumerations of valid command line options used for finite state automata.
pub enum CommandLineOption {
    Depth,
    Patterns,
    Sort,
    None
}

impl PartialEq<str> for CommandLineOption {
    fn eq(&self, rhs: &str) -> bool {
        match self {
            Self::Depth => rhs == "-l",
            Self::Patterns => rhs == "-p",
            Self::Sort => rhs == "-s",
            Self::None => false,
        }
    }
}

enum CommandLineArgType {
    Positional,
    Optional
}

/// Parses Args for valid command line options and returns a CommandLineArgs struct
/// containing provided options. Writes to stderr and exits if malformed cl-args.
pub fn parse_args(args: &[String]) -> CommandLineArgs {
    let mut cli_arguments = CommandLineArgs::default();

    if args.len() == 0 { return cli_arguments }

    if let Some(_) = args.iter().find(|i| i == &"-h" ) {
        println!("{}", HELP);
        process::exit(0);
    }

    let mut current_state = CommandLineOption::None;

    let (first, options) = args.split_first().unwrap();

    let first_arg_type = ascertain_arg_type(first);

    if let CommandLineArgType::Positional = first_arg_type {
        let dir = get_directory_from_arg(first);
        cli_arguments.set_directory(dir.to_string());
    } else {
        match ascertain_option(first) {
            Some(opt) => current_state = opt,
            None => bad_option(first)
        }
    }

    for arg in options {
        match current_state {
            CommandLineOption::None => match ascertain_option(&arg) {
                Some(opt) => current_state = opt,
                None => bad_option(&arg)
            },

            CommandLineOption::Depth => {
                validate_arg(&arg);
                let depth = get_depth_from_arg(&arg);
                cli_arguments.set_depth(depth);
                current_state = CommandLineOption::None;
            },

            CommandLineOption::Patterns => {
                validate_arg(&arg);
                cli_arguments.set_prefixes(arg.clone());
                current_state = CommandLineOption::None;
            },

            CommandLineOption::Sort => {
                validate_arg(&arg);
                let sort_type = get_sort_type_from_arg(&arg);
                cli_arguments.set_sort_type(sort_type);
                current_state = CommandLineOption::None;
            },
        }
    }

    cli_arguments
}

fn ascertain_option(flag: &str) -> Option<CommandLineOption> {
    if &CommandLineOption::Sort == flag {
        Some(CommandLineOption::Sort)
    } else if &CommandLineOption::Depth == flag {
        Some(CommandLineOption::Depth)
    } else if &CommandLineOption::Patterns == flag {
        Some(CommandLineOption::Patterns)
    } else {
        None
    }
}

fn ascertain_arg_type(arg: &str) -> CommandLineArgType {
    if arg.starts_with("-") {
        CommandLineArgType::Optional
    } else {
        CommandLineArgType::Positional
    }
}

fn get_directory_from_arg(arg: &str) -> &str {
    match fs::metadata(arg) {
        Ok(_) => arg,
        _ => {
            eprintln!("'{}' is not a valid directory.", arg);
            process::exit(1);
        }
    }
}

fn get_sort_type_from_arg(arg: &str) -> String {
    if arg == "asc" {
        "asc".to_string()
    } else if arg == "desc" {
        "desc".to_string()
    } else {
        eprintln!("-s must be <asc|desc>.");
        process::exit(1);
    }
}

fn get_depth_from_arg(arg: &str) -> u64 {
    match u64::from_str_radix(arg, 10) {
        Ok(depth) => depth,
        _ => {
            eprintln!("'{}' is not an unsigned integer.", arg);
            process::exit(1);
        }
    }
}

fn validate_arg(arg: &str) {
    if ["-s", "-l", "-p"].iter().any(|i| i == &arg) {
        eprintln!("Malformed command line arguments.");
        process::exit(1);
    }
}

fn bad_option(arg: &str) {
    eprintln!("{} is not a valid option.", arg);
    process::exit(1);
}


#[cfg(test)]
mod test {
    use super::CommandLineOption;

    #[test]
    fn test_command_line_args() {
        assert!(&CommandLineOption::Sort == "-s");
        assert!(&CommandLineOption::Sort != "-b");
        assert!(&CommandLineOption::Depth == "-l");
        assert!(&CommandLineOption::Depth != "aldsjfh");
        assert!(&CommandLineOption::Patterns == "-p");
        assert!(&CommandLineOption::Patterns != "aldsjfh");
    }
}
