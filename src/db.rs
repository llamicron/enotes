use std::collections::HashMap;
use rustbreak::{MemoryDatabase, FileDatabase, deser::Yaml};
use std::fs::File;

use crate::{NoteID, Note};

#[derive(Debug)]
pub struct DBError(String);

pub type Result<T> = std::result::Result<T, DBError>;

pub struct Database(FileDatabase<HashMap<NoteID, Note>, Yaml>);

impl Database {
    pub fn connect(file_path: &str) -> Result<Database> {
        // Will create the file if it doesn't already exist
        let db_file = File::create("./enotes.db").unwrap();
        let db = FileDatabase::<HashMap<NoteID, Note>, Yaml>::from_file(db_file, HashMap::new());
        
        db
            .map(|db|  Database(db) )
            .map_err(|err| {
                DBError(format!("Could not connect to db: {}", err))
            })
    }

    pub fn immolate(&self) -> Result<()> {
        self.0.write(|db| {
            db.clear();
        }).expect("Couldn't clear database");
        self.0.save().map(|_| () ).map_err(|err| DBError(format!("Couldn't save database: {}", err)) )
    }
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn db_connect() {
        let db = Database::connect("./enotes.db");
        assert!(db.is_ok());
    }

    #[test]
    fn test_read_write_to_db() {
        let note = Note::new(54, "note title", "note content");

        let db = Database::connect("./enotes.db").unwrap();

        db.0.write(|db| {
            db.insert(54, note);
        }).ok();

        db.0.read(|db| {
            match db.get(&54) {
                Some(found_note) => assert_eq!(found_note.title(), "note title"),
                None => assert!(false),
            }
        }).ok();

        db.immolate().unwrap();
    }

    #[test]
    fn test_delete_note() {
        let note = Note::new(54, "note title", "note content");

        let db = Database::connect("./enotes.db").unwrap();

        db.0.write(|db| {
            db.insert(54, note);
        }).ok();

        db.0.write(|db| {
            db.remove(&54);
        }).ok();

        db.0.read(|db| {
            assert!(db.get(&54).is_none());
        }).ok();

        db.immolate().unwrap();
    }
    
}
