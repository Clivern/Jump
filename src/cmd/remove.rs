// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use crate::core::kv::KvStore;
use std::path::Path;

pub fn remove(name: &str) {
    match KvStore::new(Path::new("~/.jump")) {
        Ok(kv) => kv.remove(name),
        Err(error) => panic!("Something goes wrong: {:?}.", error),
    }
}
