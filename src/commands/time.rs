use std::time::{SystemTime, UNIX_EPOCH};

use crate::command::{Command, CommandType};

struct TimeCommand {
    properties: Command,
}

impl CommandType for TimeCommand {
    fn run(&self) -> String {
        std::process::Command::new("clear").status().unwrap();
        let start = SystemTime::now();
        let since_time_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        (since_time_epoch.as_secs_f64() / 60.0 / 60.0 / 24.0).to_string()
    }

    fn get_self(&self) -> &Command {
        &self.properties
    }
}

pub fn get_command() -> Box<dyn CommandType> {
    let command = Command {
        name: "timestamp",
        description: "print the current timestamp",
        args: vec![],
    };

    let time_command = TimeCommand {
        properties: command,
    };

    Box::new(time_command)
}
