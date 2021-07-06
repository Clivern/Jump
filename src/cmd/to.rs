// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::core::kv::KvStore;
use home;

pub fn to(name: &str) {
    match home::home_dir() {
        Some(path) => {
            let full_path = format!("{}/.jump", path.display());

            match KvStore::open(full_path.as_str()) {
                Ok(kv) => match kv.get(name) {
                    Some(target) => {
                        print!("{}", target)
                    }
                    None => panic!("Error! unable to find your target path"),
                },
                Err(err) => {
                    panic!("Error: {:?}", err);
                }
            }
        }
        None => panic!("Error! unable to find your home path"),
    }
}
