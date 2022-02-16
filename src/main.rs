#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_json;

// The Rocket JSON type, used to wrap types
use rocket::serde::{json::Json};
// The serde_json Value type, a generic Json type
use serde_json::Value;

#[get("/")]
fn index() -> Value {
    json!({
        "msg": "ok!"
    })
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/0.1.0", routes![index])
}
