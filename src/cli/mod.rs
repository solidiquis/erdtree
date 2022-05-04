#![allow(dead_code)]

use std::cmp::PartialEq;
use std::env::Args;
use std::fs;
use std::process;

const HELP: &'static str = r#"
Usage:
    erdtree [options]

OPTIONS:
-d        Directory to traverse. Defaults to current working directory.
-l        Unsigned integer indicating many nested directory levels to display. Defaults to all.
-p        Comma-separated list of prefixes. Directories containing any of
          these prefixes will not be traversed. They're memory size will also be ignored.
-h        Displays help prompt.
"#;

/// Struct over valid command line options.
pub struct CommandLineArgs {
    pub directory: Option<String>,
    pub depth: Option<u64>,
    pub prefixes: Option<String>
}

impl Default for CommandLineArgs {
    fn default() -> Self {
        CommandLineArgs { directory: None, depth: None, prefixes: None }
    }
}

impl CommandLineArgs {
    fn set_directory(&mut self, directory: String) {
        self.directory = Some(directory);
    }

    fn set_depth(&mut self, depth: u64) {
        self.depth = Some(depth);
    }

    fn set_prefixes(&mut self, prefixes: String) {
        self.prefixes = Some(prefixes);
    }
}

/// Enumerations of valid command line options used for finite state automata.
enum CommandLineOption {
    Directory,
    Depth,
    Patterns,
    None
}

impl PartialEq<str> for CommandLineOption {
    fn eq(&self, rhs: &str) -> bool {
        match self {
            Self::Directory => rhs == "-d",
            Self::Depth => rhs == "-l",
            Self::Patterns => rhs == "-p",
            Self::None => false,
        }
    }
}

impl CommandLineOption {
    /// Parses Args for valid command line options and returns a CommandLineArgs struct
    /// containing provided options. Writes to stderr and exits if malformed cl-args.
    pub fn parse_args(mut args: Args) -> CommandLineArgs {
        if let Some(_) = args.find(|i| i == "-h" ) {
            println!("{}", HELP);
            process::exit(0);
        }

        let mut cli_arguments = CommandLineArgs::default();
        let mut current_state = CommandLineOption::None;

        for arg in args {
            match current_state {
                CommandLineOption::None => match Self::ascertain_option(&arg) {
                    Some(opt) => current_state = opt,
                    None => {
                        eprintln!("{} is not a valid option.", &arg);
                        process::exit(1);
                    }
                },

                CommandLineOption::Directory => {
                    Self::validate_arg(&arg);
                    let directory = Self::get_directory_from_arg(&arg);
                    cli_arguments.set_directory(directory.to_string());
                },

                CommandLineOption::Depth => {
                    Self::validate_arg(&arg);
                    let depth = Self::get_depth_from_arg(&arg);
                    cli_arguments.set_depth(depth);
                },

                CommandLineOption::Patterns => {
                    Self::validate_arg(&arg);
                    cli_arguments.set_prefixes(arg.clone());
                }
            }
        }

        cli_arguments
    }

    /// Takes a command line flag such as '-d' and tries to determine which
    /// CommandLineOption it said flag corresponds to.
    fn ascertain_option(flag: &str) -> Option<CommandLineOption> {
        if &CommandLineOption::Directory == flag {
            Some(CommandLineOption::Directory)
        } else if &CommandLineOption::Depth == flag {
            Some(CommandLineOption::Depth)
        } else if &CommandLineOption::Patterns == flag {
            Some(CommandLineOption::Patterns)
        } else {
            None
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

    fn get_depth_from_arg(arg: &str) -> u64 {
        match u64::from_str_radix(arg, 10) {
            Ok(depth) => depth,
            _ => {
                eprintln!("'{}' is not an unsigned integer.", arg);
                process::exit(1);
            }
        }
    }

    /// Ensures that cl-args are formatted properly otherwise writes
    /// to stderr and exists process.
    fn validate_arg(arg: &str) {
        if ["-d", "-l", "-p"].iter().any(|i| i == &arg) {
            eprintln!("Malformed command line arguments.");
            process::exit(1);
        }
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_command_line_option() {
        use super::CommandLineOption;

        assert!(&CommandLineOption::Directory == "-d");
        assert!(&CommandLineOption::Directory != "-b");
        assert!(&CommandLineOption::Depth == "-l");
        assert!(&CommandLineOption::Depth != "aldsjfh");
        assert!(&CommandLineOption::Patterns == "-p");
        assert!(&CommandLineOption::Patterns != "aldsjfh");
    }
}
