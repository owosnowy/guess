#![allow(special_module_name)]
use crate::prelude::*;
use crate::instance::*;
mod tools;
mod lib;
use tools::instance;
mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    println!(
        "[ G U E S S ]\nVersion {}",
        lib::VERSION
    );
    let user_input = lib::input(); 
    instance(&user_input); 
    Ok(())
}
