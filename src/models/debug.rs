use crate::models::config::ARGS;

pub struct Debug {}

impl Debug {
     pub fn print(message: String) {
        if ARGS.debug {
            eprintln!("{}", message);
        }
    }
}