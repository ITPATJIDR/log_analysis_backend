use controllers::log_controller::analyze_log;
#[macro_use] extern crate rocket;
mod controllers;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![analyze_log])
}
