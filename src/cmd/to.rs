// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use inquire::Text;

pub fn to() {
    let name = Text::new("What is the project name?").prompt();

    match name {
        Ok(name) => {
            println!("Name = {}", name);
        }
        Err(_) => {
            println!("Oops! An error happened.")
        }
    }
}
