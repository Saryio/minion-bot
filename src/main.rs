use std::collections::HashMap;

use command::CommandType;
use controller::Controller;
use rust_bot::auto_define;
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
    println!(
        "Minion-B says:\n{}",
        controller.commands[input.trim()].run()
    );
}

fn build_commands(controller: &mut Controller) {

    let mut cmds: Vec<Box<dyn CommandType>> = vec![];

    auto_define!(commands);

    for command in cmds {
        controller.commands.insert(command.get_self().name, command);
    }
}

fn ask_command(input: &mut String, controller: &Controller) {
    println!("Minion-B says:\nSelect your command:\n");
    for command in controller.commands.keys() {
        println!("{}: {}", command.to_string(), controller.commands[command].get_self().description);
    }
    println!();
    std::io::stdin().read_line(input).unwrap();
}