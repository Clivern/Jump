// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::core::kv::KvStore;
use home;
use inquire::Text;

pub fn new(name: &str) {
    let new_path = Text::new("What is the project path?").prompt();

    match new_path {
        Ok(new_path) => match home::home_dir() {
            Some(path) => {
                let full_path = format!("{}/.jump", path.display());

                match KvStore::open(full_path.as_str()) {
                    Ok(mut kv) => {
                        kv.set(String::from(name), String::from(new_path));
                        match kv.flush(full_path.as_str()) {
                            Ok(_) => {}
                            Err(err) => panic!("Error: {:?}", err),
                        }
                    }
                    Err(err) => {
                        panic!("Error: {:?}", err);
                    }
                }
            }
            None => panic!("Error! unable to find your home path"),
        },
        Err(_) => {
            println!("Oops! An error happened.")
        }
    }
}
