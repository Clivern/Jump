// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

mod cmd;
mod core;
use crate::core::kv::KvStore;
use clap::{arg, Command};
use cmd::new::new;
use cmd::remove::remove;
use cmd::to::to;
use home;
use std::path::Path;

fn main() {
    match home::home_dir() {
        Some(path) => {
            let full_path = format!("{}/.jump", path.display());

            if !Path::new(full_path.as_str()).exists() {
                let mut kv = KvStore::new();
                kv.set(String::from("home"), format!("{}", path.display()));

                match kv.flush(full_path.as_str()) {
                    Ok(_) => {}
                    Err(err) => panic!("Error {:?}", err),
                }
            }
        }
        None => panic!("Error! unable to find your home path"),
    }

    let matches = Command::new("ju")
        .about("A Command Line Tool for Fast System Navigation in Rust")
        .version("0.2.0")
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
