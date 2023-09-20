use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, guard};
use rust_auth::auth::get_profile;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").configure(get_profile))
    })
    .bind(("127.0.0.1", 8080))?
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
