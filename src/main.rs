#[macro_use]
extern crate rocket;
use std::vec;
use kquotes::constants;
use kquotes::quotes_route;

#[catch(404)]
fn not_found() -> String {
    constants::NOT_IMPLEMENTED.to_owned()
}

#[launch]
fn rocket() -> _ {

    let routes = routes![
        quotes_route::quotes
    ];

    let catchers = catchers![
        not_found
    ];


    rocket::build()
        .mount(constants::ROOT, routes)
        .register(constants::ROOT, catchers)
}
