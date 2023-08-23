use my_basic_cli_tools::Command;
use std::{
    error::Error,
    io::{self, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    loop {
        print!("> ");

        // cleanup
        io::stdout().flush()?;
        input.clear();

        io::stdin().read_line(&mut input)?;

        let command = match Command::try_from(input.trim().to_owned()) {
            Ok(command) => command,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };

        if let Err(e) = command.execute() {
            println!("An error occured: {}", e);
        }
    }
}
