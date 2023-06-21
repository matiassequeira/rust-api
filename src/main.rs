mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use env_logger::Env;

//add imports below
use api::user_api::{create_user, get_user, update_user, delete_user, get_all_user};
use api::product_api::{create_product, get_product, update_product, delete_product, get_all_product};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();


    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/user", routes![
            create_user, 
            get_user, 
            update_user,
            delete_user,
            get_all_user
        ])
        .mount("/product", routes![
            create_product, 
            get_product, 
            update_product, 
            delete_product, 
            get_all_product
        ])
}