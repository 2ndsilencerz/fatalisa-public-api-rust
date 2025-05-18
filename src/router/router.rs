use crate::service::general::general;
use actix_web::{web, App, HttpServer};
use std::string::String;
use crate::service::qris::qris::qris_mpm_get;

#[actix_web::main]
pub async fn router(addr: String, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(general::hello))
            .route("echo", web::post().to(general::echo))
            .route("/hey", web::get().to(general::manual_hello))
            .route("/api/qris/mpm/{raw}", web::get().to(|path: web::Path<String>| qris_mpm_get(path.into_inner())))
    })
        .bind(format!("{}:{}", addr, port))?
        .run()
        .await
}