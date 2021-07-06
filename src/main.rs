// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use clap::Command;
use inquire::Text;

fn main() {
    let matches = Command::new("ju")
        .about("A Command Line Tool for Fast System Navigation in Rust")
        .version("0.1.2")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Clivern")
        .subcommand(
            Command::new("new")
                .short_flag('n')
                .long_flag("new")
                .about("Add a new project path."),
        )
        .subcommand(
            Command::new("to")
                .short_flag('t')
                .long_flag("to")
                .about("Go to a project path."),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("new", _)) => {
            let name = Text::new("What is the project name?").prompt();

            match name {
                Ok(name) => {
                    println!("Hello {}", name)
                }
                Err(_) => println!("Oops! An error happened."),
            }
        }
        Some(("to", _)) => {}
        _ => unreachable!(),
    }
}
