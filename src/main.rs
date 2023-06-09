extern crate rocket;
use rocket::{launch, routes};
pub mod models;
pub mod schema;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![services::create_post, services::hello_world])
}
