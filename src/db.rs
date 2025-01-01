#[derive(Debug)]
pub struct DbRecord {
    pub key: String,
    pub value: String,
}


impl DbRecord {
    pub fn new(k: String, v: String) -> DbRecord {
        DbRecord { key: k, value: v }
    }
}

pub trait WriteAheadLog {
    fn append(self, record: DbRecord) -> bool;
    fn recover(self);
}

pub trait Database {
    fn get(&mut self, key: String) -> Option<DbRecord>;
    fn put(&mut self, key: String, value: String) -> Option<DbRecord>;
    fn version(&mut self);
    fn del(&mut self, key: String) -> Option<DbRecord>;
}
