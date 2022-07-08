// pub trait Command {
//     fn run(&self) -> String;
// }

// #[derive(Debug)]
// pub struct Properties {
//     pub name: &'static str,
//     pub description: &'static str,
//     pub args: Vec<&'static str>,
// }
pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub args: Vec<&'static str>,
}

pub trait CommandType {
    fn run(&self) -> String;
    fn get_self(&self) -> &Command;
}
