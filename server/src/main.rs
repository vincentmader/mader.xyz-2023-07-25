#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    "Hello, rocket.rs server!".into()
}

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, index])
}
