#[macro_use]
extern crate rocket;
use kquotes::constants;
use kquotes::quotes_route;
use std::vec;

#[catch(404)]
fn not_found() -> String {
    constants::NOT_IMPLEMENTED.to_owned()
}

#[catch(500)]
fn internal_error() -> String {
    constants::NOT_IMPLEMENTED.to_owned()
}

#[launch]
fn rocket() -> _ {
    let routes = routes![quotes_route::quotes];

    let catchers = catchers![not_found, internal_error,];

    rocket::build()
        .mount(constants::ROOT, routes)
        .register(constants::ROOT, catchers)
}
