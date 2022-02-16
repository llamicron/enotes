#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;


// The Rocket JSON type, used to wrap types
#[allow(unused)]
use rocket::serde::{json::Json};
// The serde_json Value type, a generic Json type
use serde_json::Value;


// A basic route to test if the API is running. Should return with an Ok message
#[get("/")]
fn index() -> Value {
    json!({
        "msg": "ok!"
    })
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": i32::from(404),
        "msg": "route not found"
    })
}


#[launch]
fn launch() -> _ {
    rocket::build().register("/api/0.1.0", catchers![not_found]).mount("/api/0.1.0", routes![index])
}
