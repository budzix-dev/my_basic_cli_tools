mod input_utils;

use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Command {
    pub command_type: CommandType,
    pub arguments: Vec<String>,
    pub flags: Vec<String>,
}

impl Command {
    pub fn new(
        command_type: CommandType,
        arguments: Vec<String>,
        flags: Vec<String>,
    ) -> Result<Self, CommandError> {
        for flag in flags.iter() {
            if !command_type.is_supported_flag(flag) {
                return Err(CommandError::UnsupportedFlag(flag.to_owned()));
            }
        }

        let expected_argument_count = command_type.get_expected_argument_count();

        if let Some(expected_argument_count) = expected_argument_count {
            let actual_argument_count = arguments.len();

            if !expected_argument_count.is_valid(actual_argument_count) {
                return Err(CommandError::WrongArgumentsCount {
                    expected: expected_argument_count,
                    actual: actual_argument_count,
                });
            }
        }

        Ok(Self {
            command_type,
            arguments,
            flags,
        })
    }

    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        match &self.command_type {
            CommandType::Echo => {
                println!("{}", self.arguments.join("\n"));
            }
            CommandType::Exit => {
                std::process::exit(0);
            }
        }

        Ok(())
    }
}

impl TryFrom<String> for Command {
    type Error = CommandError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        let input_vec = input_utils::split_input_outside_quotes_on_whitespace(input);

        let command_type = CommandType::try_from(input_vec[0].to_owned())?;

        let mut arguments = Vec::new();
        let mut flags = Vec::new();

        for arg in input_vec.iter().skip(1) {
            if arg.starts_with('-') {
                flags.push(arg.to_owned());
            } else {
                arguments.push(arg.to_owned());
            }
        }

        Self::new(command_type, arguments, flags)
    }
}

#[derive(Debug)]
pub enum CommandType {
    Echo,
    Exit,
}

impl CommandType {
    fn get_supported_flags(&self) -> Vec<&str> {
        match self {
            CommandType::Echo => vec!["-u"],
            CommandType::Exit => vec![],
        }
    }

    fn is_supported_flag(&self, flag: &str) -> bool {
        self.get_supported_flags().contains(&flag)
    }

    fn get_expected_argument_count(&self) -> Option<ArgumentCount> {
        match self {
            CommandType::Echo => Some(ArgumentCount::AtLeast(1)),
            CommandType::Exit => Some(ArgumentCount::Exact(0)),
        }
    }
}

impl TryFrom<String> for CommandType {
    type Error = CommandError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        match input.as_str() {
            "echo" => Ok(CommandType::Echo),
            "exit" => Ok(CommandType::Exit),
            _ => Err(CommandError::UnknownCommand(input)),
        }
    }
}

#[derive(Debug)]
pub enum ArgumentCount {
    Exact(usize),
    AtLeast(usize),
    AtMost(usize),
    Range(usize, usize),
}

impl ArgumentCount {
    pub fn is_valid(&self, count: usize) -> bool {
        match self {
            ArgumentCount::Exact(expected) => count == *expected,
            ArgumentCount::AtLeast(expected) => count >= *expected,
            ArgumentCount::AtMost(expected) => count <= *expected,
            ArgumentCount::Range(min, max) => count >= *min && count <= *max,
        }
    }
}

impl Display for ArgumentCount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ArgumentCount::Exact(count) => write!(f, "exactly {}", count),
            ArgumentCount::AtLeast(count) => write!(f, "at least {}", count),
            ArgumentCount::AtMost(count) => write!(f, "up to {}", count),
            ArgumentCount::Range(min, max) => write!(f, "{}-{}", min, max),
        }
    }
}

#[derive(Debug)]
pub enum CommandError {
    UnknownCommand(String),
    UnsupportedFlag(String),
    WrongArgumentsCount {
        expected: ArgumentCount,
        actual: usize,
    },
}

impl Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommandError::UnknownCommand(command) => write!(f, "Unknown command: {}", command),
            CommandError::UnsupportedFlag(flag) => write!(f, "Unsupported flag: {}", flag),
            CommandError::WrongArgumentsCount { expected, actual } => write!(
                f,
                "Wrong number of arguments: expected {}, got {}",
                expected, actual
            ),
        }
    }
}
