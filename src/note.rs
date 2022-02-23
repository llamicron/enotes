use serde::{Serialize, Deserialize};

// Type alias, this makes it easier to change the note id type later
pub type NoteID = u32;

// The definition for a Note
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Note {
    id: Option<NoteID>,
    title: String,
    content: String,
}

impl Note {
    pub fn new(id: NoteID, title: &str, content: &str) -> Note {
        Note { id: Some(id), title: String::from(title), content: String::from(content) }
    }
    
    pub fn id(&self) -> &Option<NoteID> {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn test_note() -> Note {
        Note {
            id: Some(0),
            title: String::from("Testing note"),
            content: String::from("Content of the note. This will be really long")
        }
    }

    #[test]
    fn test_create_note() {
        let note = test_note();

        assert_eq!(note.id, Some(0));
        assert_eq!(note.title, "Testing note");
    }

    #[test]
    fn test_note_to_json() {
        // Create raw json of a note, mirroring the testing note
        let json = json!({
            "id": 0,
            "title": "Testing note",
            "content": "Content of the note. This will be really long"
        });

        // Get a test note struct
        let note = test_note();
        // Turn that struct into json
        let json2 = serde_json::to_value(note).unwrap();
        // Compare actual and expected values
        assert_eq!(json, json2);
    }

    #[test]
    fn test_note_from_json() {
        // Create a Note struct
        let note = test_note();
        // Create raw json of a Note
        let json = json!({
            "id": 0,
            "title": "Testing note",
            "content": "Content of the note. This will be really long"
        });

        // Ensure we can deserialize json to a Note struct
        let note2: Note = serde_json::from_value(json).unwrap();

        // Compare expected and actual values
        assert_eq!(note, note2);
    }
}
