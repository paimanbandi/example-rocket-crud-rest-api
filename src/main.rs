#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket_contrib::json::Json;
use serde_json::{Value, json};

mod user;
use user::{User};

pub mod schema;
pub mod db;

#[post("/", data = "<user>")]
fn create(user: Json<User>, connection: db::Connection) -> Json<User> {
    let insert = User { ..user.into_inner() };
    Json(User::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> Json<Value> {
    Json(json!(User::read(&connection)))
}

#[put("/<id>", data = "<user>")]
fn update(id: i32, user: Json<User>, connection: db::Connection) -> Json<Value> {
    let update = User { ..user.into_inner() };
    Json(json!({
        "success": User::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": User::delete(id, &connection)
    }))
}

fn main() {
    rocket::ignite()
        .mount("/user", routes![create, update, delete])
        .mount("/users", routes![read])
        .manage(db::connect())
        .launch();
}
