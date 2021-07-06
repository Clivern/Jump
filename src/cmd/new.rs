// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::core::kv::KvStore;
use inquire::Text;
use std::path::Path;

pub fn new(name: &str) {
    let path = Text::new("What is the project path?").prompt();

    match path {
        Ok(path) => match KvStore::new(Path::new("~/.jump")) {
            Ok(kv) => kv.set(name.to_string(), path.to_string()),
            Err(error) => panic!("Something goes wrong: {:?}.", error),
        },
        Err(_) => {
            println!("Oops! An error happened.")
        }
    }
}
