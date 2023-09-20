use actix_web::{web, App, HttpResponse, HttpServer};

pub fn get_profile (cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/profile")
        .route(web::get().to(|| async { HttpResponse::Ok().body("apfdsafdsafdfjdklsajfl;dsjafsfdsp")}))
    );
}