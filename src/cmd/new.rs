// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use inquire::Text;

pub fn new() {
    let name = Text::new("What is the project name?").prompt();

    match name {
        Ok(name) => {
            println!("Name = {}", name);

            let path = Text::new("What is the project path?").prompt();

            match path {
                Ok(path) => {
                    println!("Path = {}", path)
                }
                Err(_) => {
                    println!("Oops! An error happened.")
                }
            }
        }
        Err(_) => {
            println!("Oops! An error happened.")
        }
    }
}
