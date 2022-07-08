pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub args: Vec<&'static str>,
}

pub trait CommandType {
    fn run(&self) -> String;
    fn get_self(&self) -> &Command;
}
