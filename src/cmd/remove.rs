// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::core::kv::KvStore;
use home;

pub fn remove(name: &str) {
    match home::home_dir() {
        Some(path) => {
            let full_path = format!("{}/.jump", path.display());

            match KvStore::open(full_path.as_str()) {
                Ok(mut kv) => {
                    kv.remove(name);
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
    }
}
