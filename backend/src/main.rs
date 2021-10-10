#[macro_use]
extern crate rocket;
mod block;
mod blockchain;
mod routes;
use blockchain::Blockchain;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(routes::stage())
}
