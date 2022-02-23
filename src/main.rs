#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;

// The Rocket JSON type, used to wrap types
#[allow(unused)]
use rocket::serde::{json::Json};

pub mod note;
use note::{Note, NoteID};

// Basic routes ------

// A basic route to test if the API is running. Should return with an Ok message
#[get("/")]
fn index() -> serde_json::Value {
    json!({
        "msg": "ok!"
    })
}

#[catch(404)]
fn not_found() -> serde_json::Value {
    json!({
        "status": i32::from(404),
        "msg": "route not found"
    })
}


// Resource management ------

#[get("/note/<_id>")]
fn get_note(_id: NoteID) -> serde_json::Value {
    // Retrieve the note from the database/storage
    // Hardcoded note for testing
    json!({
        "id": 0,
        "title": "Testing note",
        "content": "Here's the content of the note"
    })
}

#[post("/note", format = "json", data = "<json_note>")]
fn create_note(json_note: Json<Note>) -> serde_json::Value {
    let note = json_note.into_inner();

    json!({
        "id": 1,
        "title": note.title(),
        "content": note.content()
    })
}



#[launch]
fn launch() -> _ {
    rocket::build()
        .register("/api/0.1.0", catchers![not_found])
        .mount("/api/0.1.0", routes![index, get_note, create_note])
}
