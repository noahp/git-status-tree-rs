extern crate colored;
extern crate git2;

use colored::*;
use git2::{Repository, Error, StatusOptions, ErrorCode, SubmoduleIgnore};

fn main() {
    println!("{}", "Hello, world!".blue().bold().on_color("white"));
}
