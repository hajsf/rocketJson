#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::io;

use rocket::request::{Form, FormError, FormDataError};
use rocket::response::NamedFile;
use rocket::http::RawStr;

#[derive(Debug, FromFormValue)]
enum FormOption {
    A, B, C
}

#[derive(Debug, FromForm)]
struct FormInput<'r> {
    checkbox: bool,
    number: usize,
    #[form(field = "type")]
    radio: FormOption,
    password: &'r RawStr,
    #[form(field = "textarea")]
    text_area: String,
    select: FormOption,
}

use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json::JsonError;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    checkbox: bool,
    number: usize,
    radio: String,
    password: String,
    text_area: String,
    select: String,
}

#[post("/", data = "<json>")]
fn sink(json: Result<Json<Message>, JsonError>) -> JsonValue {
    match json {
        Ok(value) => {
            println!("The value: {:?}", value);
            println!("The value: {:?}", value.0);
            println!("The value: {}", value.text_area);
            println!("The value: {}", value.0.text_area);
            json!({
                "status": "Success",
                "reason": format!("The value: {}", value.text_area)
            })
        },
        Err(JsonError::Io(e)) => {
          //  println!("I/O Error: {:?}", e)
            json!({
                "status": "error",
                "reason": format!("I/O Error: {:?}", e)
            })
        },
        Err(JsonError::Parse(raw, e)) => {
           // println!("{:?} is not valid JSON: {:?}", raw, e)
            json!({
                "status": "error",
                "reason": format!("{:?} is not valid JSON: {:?}", raw, e)
            })
        },
    }
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, sink])
}

fn main() {
    rocket().launch();
}
