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

#[get("/note/<_id>")]
fn get_note(_id: NoteID) -> serde_json::Value {
    // Retrieve the note from the database/storage
    // Hardcoded note for testing
    json!({
        "status": i32::from(200),
        "msg": "",
        "data": {
            "id": 0,
            "title": "Testing note",
            "content": "Here's the content of the note"
        }
    })
}

#[post("/note", format = "json", data = "<json_note>")]
fn create_note(json_note: Json<Note>) -> serde_json::Value {
    let note = json_note.into_inner();

    json!({
        "status": i32::from(200),
        "msg": "",
        "data": {
            "id": 1,
            "title": note.title(),
            "content": note.content()
        }
    })
    
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
    rocket::build()
        .register("/api/0.1.0", catchers![not_found])
        .mount("/api/0.1.0", routes![index, get_note, create_note, update_note, delete_note])
}
