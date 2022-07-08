use crate::command::{Command, CommandType};

struct PrintCommand {
    properties: Command,
}

impl CommandType for PrintCommand {
    fn run(&self) -> String {
        std::process::Command::new("clear").status().unwrap();
        println!("Minion-B says:\nType what you want to be printed:\n");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        std::process::Command::new("clear").status().unwrap();
        input
    }

    fn get_self(&self) -> &Command {
        &self.properties
    }
}

pub fn get_command() -> Box<dyn CommandType> {
    let command = Command {
        name: "print",
        description: "print some thing",
        args: vec![],
    };

    let print_command = PrintCommand {
        properties: command,
    };

    Box::new(print_command)
}
