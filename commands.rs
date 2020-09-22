mod lib;
pub use crate::lib;

pub enum Command {
    ListAll,
    Add(String),
}

impl Command {
    pub fn new(arg: &Vec<String>) -> Result<Command, String> {
        if arg.is_empty() {
            return Ok(Command::ListAll);
        }
        // if arg.len() == 1 {
        //     return Err("missing argument".to_string());
        // }
        match arg[0].to_lowercase().as_ref() {
            "listall" => Ok(Command::ListAll),
            "add" => Ok(Command::Add(arg[1].clone())),
            _ => return Err("undefiend command".to_string()),
        }
    }
}