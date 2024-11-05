use crate::models::config::ARGS;

pub struct Debug {}

impl Debug {
    #[allow(dead_code)]
     pub fn print(message: String) {
        if ARGS.debug {
            eprintln!("{}", message);
        }
    }
}