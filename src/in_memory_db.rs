use crate::db::{Database, DbRecord, WriteAheadLog};
use std::collections::HashMap;

#[derive(Debug)]
pub struct InMemoryDb {
    engine: String,
    wal: InMemoryWal,
    map: HashMap<String, String>,
}

#[derive(Debug)]
pub struct InMemoryWal {
    data: Vec<DbRecord>,
}

impl WriteAheadLog for InMemoryWal {
    fn append(mut self, record: DbRecord) -> bool {
        self.data.push(record);
        return true;
    }

    fn recover(mut self) {
        println!("HashMap recovered");
    }
}

impl InMemoryWal {
    fn new() -> InMemoryWal {
        InMemoryWal { data: Vec::new() }
    }
}

impl InMemoryDb {
    pub fn new() -> InMemoryDb {
        InMemoryDb {
            engine: "hashmap".to_string(),
            wal: InMemoryWal::new(),
            map: HashMap::new(),
        }
    }

    pub fn with_engine(mut self, engine: &str) -> InMemoryDb {
        self.engine = engine.to_string();
        self
    }
}

impl Database for InMemoryDb {
    fn get(&mut self, key: String) -> Option<DbRecord> {
        if self.map.contains_key(&key) {
            let value_from_db = self.map.get(&key).unwrap();
            println!("Record with key {} is fetch from database", key);
            return Some(DbRecord::new(key.to_string(), value_from_db.to_string()));
        }
        println!("Record with key {} does not exist in database", key);
        return None;
    }

    fn put(&mut self, key: String, value: String) -> Option<DbRecord> {
        let return_value = Some(DbRecord::new(key.clone(), value.clone()));
        println!(
            "Record with key {} and value{} is written to database",
            key, value
        );
        self.map.insert(key, value);

        return_value
    }

    fn version(&mut self) {
        println!("Version 1.0");
    }

    fn del(&mut self, key: String) -> Option<DbRecord> {
        let value_from_map = self.map.remove(&key);
        let return_value = Some(DbRecord::new(key.clone(), value_from_map.unwrap().clone()));
        return_value
    }
}
