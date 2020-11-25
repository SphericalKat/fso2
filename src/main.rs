#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

pub mod api;
pub mod models;
pub mod schemas;

fn rocket() -> rocket::Rocket {
    let rocket = rocket::ignite();
    api::endpoints::fuel(rocket)
}

fn main() {
    // Load env variables
    dotenv::dotenv().ok();

    // Launch rocket instance
    rocket().launch();
}
