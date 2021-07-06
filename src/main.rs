// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

mod cmd;
mod core;
use clap::{arg, Command};
use cmd::new::new;
use cmd::remove::remove;
use cmd::to::to;

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
                .about("Add a new project path.")
                .arg(arg!(<NAME> "The path name"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("to")
                .short_flag('t')
                .long_flag("to")
                .about("Go to a project path.")
                .arg(arg!(<NAME> "The path name"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("remove")
                .short_flag('r')
                .long_flag("remove")
                .about("Remove a project.")
                .arg(arg!(<NAME> "The path name"))
                .arg_required_else_help(true),
        )
        .get_matches();

    match matches.subcommand() {
        // new command
        Some(("new", sub_matches)) => match sub_matches.get_one::<String>("NAME") {
            Some(name) => new(name),
            _ => println!("Oops! invalid name provided."),
        },
        // to command
        Some(("to", sub_matches)) => match sub_matches.get_one::<String>("NAME") {
            Some(name) => to(name),
            _ => println!("Oops! invalid name provided."),
        },
        // remove command
        Some(("remove", sub_matches)) => match sub_matches.get_one::<String>("NAME") {
            Some(name) => remove(name),
            _ => println!("Oops! invalid name provided."),
        },
        _ => unreachable!(),
    }
}
