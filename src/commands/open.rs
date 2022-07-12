use crate::command::{Command, CommandType};

struct OpenCommand {
    properties: Command,
}

impl CommandType for OpenCommand {
    fn run(&self) -> String {
        std::process::Command::new("clear").status().unwrap();
        println!("Minion-B says:\nType what you want to be opened:\n");
        let mut input = String::new();
        std::io::stdin().readline(&mut input).unwrap();
        let result = match std::process::Command::new(input.trim()).status() {
            Ok() => "Opening ".tostring() + input.trim(),
            Err() => "Failed to open ".to_string() + input.trim(),
        };
        std::process::Command::new("clear").status().unwrap();
        result
    }

    fn get_self(&self) -> &Command {
        &self.properties
    }
}

pub fn get_command() -> Box<dyn CommandType> {
    let command = Command {
        name: "open",
        description: "open what you want to be opened",
        args: vec![],
    };

    let print_command = OpenCommand {
        properties: command,
    };

    Box::new(print_command)
}