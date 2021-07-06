// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct KvStore {
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn remove(&mut self, key: &str) {
        self.store.remove(key);
    }

    pub fn flush(&self, filename: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self)?;
        let mut file = File::create(filename)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn open(filename: &str) -> std::io::Result<KvStore> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let kv_store = serde_json::from_str(&contents)?;
        Ok(kv_store)
    }
}
