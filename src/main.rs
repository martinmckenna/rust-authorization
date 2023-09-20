use actix_web::{get, guard, post, web, App, HttpResponse, HttpServer, Responder};
use rust_auth::auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("").configure(auth::get_profile)))
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}

// mod front_of_house {
// }

// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// /* src/front-of-house.rs */
// pub mod hosting {
//     pub fn add_to_waitlist() {}
// }
