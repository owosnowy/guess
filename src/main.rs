use crate::prelude::*;
use crate::instance::*;
mod tools;
mod lib;
use tools::instance;
mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    let user_input = lib::input(); 
    instance(&user_input); 
    Ok(())
}
