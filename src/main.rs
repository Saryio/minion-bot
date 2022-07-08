use std::collections::HashMap;

use command::CommandType;
use controller::Controller;
mod command;
mod commands;
mod controller;
fn main() {
    let mut controller = Controller {
        commands: HashMap::new(),
    };
    build_commands(&mut controller);
    let mut input = String::new();
    ask_command(&mut input, &controller);
    println!("Minion-B says:\n{}", controller.commands[input.trim()].run());
}

fn build_commands(controller: &mut Controller) {
    // Commands declaration
    let print = commands::print::get_command();
    let time = commands::time::get_command();

    // Only vector of commands
    let mut commands: Vec<Box<dyn CommandType>> = vec![];

    // Push the commands to the vector
    commands.push(print);
    commands.push(time);

    for command in commands {
        controller.commands.insert(command.get_self().name, command);
    }
}

fn ask_command(input: &mut String, controller: &Controller) {
    println!("Minion-B says:\nSelect your command:\n");
    for command in controller.commands.keys() {
        println!("{}", command.to_string());
    }
    println!();
    std::io::stdin().read_line(input).unwrap();
}