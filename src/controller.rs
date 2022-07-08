use std::collections::HashMap;

use crate::command::CommandType;
pub struct Controller<'a> {
    pub commands: HashMap<&'a str, Box<dyn CommandType>>,
}
