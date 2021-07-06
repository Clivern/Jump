// Copyright 2022 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

struct KvStore {
    store: HashMap<String, String>,
    file: Option<BufWriter<File>>,
}

impl KvStore {
    fn new(path: &Path) -> Result<KvStore, std::io::Error> {
        let file = OpenOptions::new().read(true).write(true).create(true).open(path)?;
        let reader = BufReader::new(file.try_clone()?);
        let mut store = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                store.insert(parts[0].to_owned(), parts[1].to_owned());
            }
        }

        Ok(KvStore {
            store,
            file: Some(BufWriter::new(file)),
        })
    }

    fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
        if let Some(file) = &mut self.file {
            writeln!(file, "{}={}", key, value).expect("Failed to write to file");
        }
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    fn remove(&mut self, key: &str) {
        self.store.remove(key);
        if let Some(file) = &mut self.file {
            let data = self.store.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<String>>().join("\n");
            file.seek(std::io::SeekFrom::Start(0)).expect("Failed to seek to start of file");
            file.write_all(data.as_bytes()).expect("Failed to write to file");
            file.flush().expect("Failed to flush file");
        }
    }
}

