use rocket::Rocket;
use rocket_cors::CorsOptions;

pub mod files;

pub fn fuel(rocket: Rocket) -> Rocket {
    let mut rocket = rocket;
    let cors = CorsOptions::default().to_cors().unwrap();
    rocket = files::fuel(rocket);

    rocket.attach(cors)
}