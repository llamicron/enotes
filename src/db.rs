use std::{collections::HashMap, fs, io::Write, path::Path};
use crate::note::{NoteID, Note};

#[derive(Debug)]
pub struct DBError(String);

type Result<T> = std::result::Result<T, DBError>;
type NoteCollection = HashMap<NoteID, Note>;

#[derive(Debug)]
pub struct Database {
    inner: NoteCollection,
    file_path: String
}

impl Database {
    pub fn connect(file_path: &str) -> Result<Database> {
        if !Path::new(file_path).exists() {
            let file = fs::File::create(file_path).map_err(|e| {
                DBError(format!("Could not create db file: {}", e))
            })?;
        }

        let file_contents = fs::read_to_string(file_path).expect("Could not read from db file");
        let inner = serde_json::from_str::<NoteCollection>(&file_contents).unwrap();
        
        Ok(Database {
            inner,
            file_path: String::from(file_path)
        })
    }

    fn persist(&mut self) -> Result<()> {
        let data = serde_json::to_string(&self.inner)
            .map_err(|e| DBError(format!("{}", e) ))?;

        let mut file = fs::File::options().write(true).open(&self.file_path).unwrap();
        file.write_all(data.as_bytes()).map_err(|e| {
            DBError(format!("Could not persist database: {}", e))
        })
    }

    fn wipe(&mut self) -> Result<()> {
        self.inner.clear();
        self.persist()?;
        Ok(())
    }

    pub fn all_notes(&mut self) -> &NoteCollection {
        &self.inner
    }

    pub fn get_note(&mut self, id: NoteID) -> Result<&Note> {
        self.inner.get(&id).ok_or(DBError(format!("Note {} not found", id)))
    }

    pub fn update_note(&mut self, id: NoteID, mut new_note: Note) -> Result<Note> {
        if self.inner.get(&id).is_some() {
            let updated = new_note.clone();
            self.inner.insert(id, updated);
            self.persist().ok();
            Ok(new_note)
        } else {
            Err(DBError(format!("Note {} not found", id)))
        }
    }

    pub fn create_note(&mut self, note: Note) -> Result<Note> {
        let keys = &mut self.inner.keys().collect::<Vec<&NoteID>>();
        let highest = keys.iter().max();
        let id: NoteID = if highest.is_none() { 0 } else { *highest.unwrap() + 1 };

        let new_note = Note::new(id, note.title(), note.content());
        let returned = new_note.clone();
        self.inner.insert(id, new_note);

        self.persist().ok();

        Ok(note)
    }

    pub fn delete_note(&mut self, id: NoteID) -> Result<()> {
        let result = match self.inner.remove(&id) {
            Some(_) => Ok(()),
            None => Err(DBError(format!("Couldn't delete note")))
        };
        self.persist().ok();
        result
    }
}








#[cfg(test)]
mod tests {
    use super::*;

    fn db() -> Database {
        Database::connect("./enotes.db").unwrap()
    }

    #[test]
    fn test_db_connection() {
        let db = Database::connect("./enotes.db");
        assert!(db.is_ok());
    }

    #[test]
    fn test_persist() {
        let mut db = db();
        let note = Note::new(8, "title", "content");
        db.inner.insert(8, note);
        db.persist().ok();
    }

    #[test]
    fn test_get_all_notes() {
        let mut db = db();
        assert!(db.all_notes().len() > 0);
    }

    #[test]
    fn test_get_note() {
        let mut db = db();
        let note = db.get_note(6);
        assert!(note.is_ok());
        let not_found = db.get_note(12313);
        assert!(not_found.is_err());
    }
}
