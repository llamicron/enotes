#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_variables, unused_imports)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;

extern crate rustbreak;

// The Rocket JSON type, used to wrap types
#[allow(unused)]
use rocket::serde::{json::Json};
use rocket::State;

pub mod db;
pub mod note;

use db::{Database};
use note::{Note, NoteID};

// This function is garbage
// I should attach the DB as a State<Database> attachment to
// rocket but I can't implement FromRequest for the rustbreak DB.
// I should slog through the lifetime shit or use a real database.
// I'll just connect to the db for each request. This will *not* work at
// scale but it'll work for now.
fn db() -> Database {
    Database::connect("./enotes.db").expect("Couldn't connect to the DB")
}

// Basic routes ------

// A basic route to test if the API is running. Should return with an Ok message
#[get("/")]
fn index() -> serde_json::Value {
    json!({
        "status": i32::from(200),
        "msg": "ok!"
    })
}


#[catch(404)]
fn not_found() -> serde_json::Value {
    json!({
        "status": i32::from(404),
        "msg": "route not found",
        "data": {}
    })
}


// Resource management ------

#[get("/note/<id>")]
fn get_note(id: NoteID) -> serde_json::Value {
    // Retrieve the note from the database/storage
    // Hardcoded note for testing

    let db = db();
    db.0.read(|db| {
        if let Some(note) = db.get(&id) {
            return json!({
                "status": i32::from(200),
                "msg": "",
                "data": serde_json::to_string(&note).unwrap()
            });
        } else {
            return json!({
                "status": i32::from(404),
                "msg": format!("note not found at id {}", id)
            });
        }
    }).unwrap();

    return json!({
        "status": i32::from(404),
        "msg": format!("note not found at id {}", id)
    });
}

#[post("/note", format = "json", data = "<json_note>")]
fn create_note(json_note: Json<Note>) -> serde_json::Value {
    let db = db();

    db.0.write(|db| {
        let highest = db.iter().max_by(|a, b| a.0.cmp(&b.0)).map(|(k, _v)| k).unwrap();
        let id = highest + 1;

        let mut note = json_note.into_inner();
        note.id = Some(id);
        db.insert(id, note);
    }).ok();

    db.0.save();

    return json!({
        "status": i32::from(404),
        "msg": format!("note not found at id {}", id)
    });
    
}

#[put("/note/<_id>", format = "json", data = "<json_note>")]
fn update_note(_id: NoteID, json_note: Json<Note>) -> serde_json::Value {
    let note = json_note.into_inner();
    

    // find the note in the db by id (the id variable) and update it
    // with the content of the PUTed note

    println!("{:#?}", note);

    json!({
        "status": i32::from(200),
        "msg": "note updated",
        "data": {
            "id": note.id().unwrap(),
            "title": note.title(),
            "content": note.content()
        }
    })
}

#[delete("/note/<_id>")]
fn delete_note(_id: NoteID) -> serde_json::Value {
    // Delete the note from the DB
    json!({
        "status": i32::from(200),
        "msg": "note deleted",
        "data": {}
    })
}



#[launch]
fn launch() -> _ {
    let db = db::Database::connect("./enotes.db")
        .expect("Could not connect to database");

    rocket::build()
        .register("/api/0.1.0", catchers![not_found])
        .manage(db)
        .mount("/api/0.1.0", routes![index, get_note, create_note, update_note, delete_note])
}
