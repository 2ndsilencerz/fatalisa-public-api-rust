use actix_web::{HttpResponse, Responder};
use crate::service;

pub(crate) async fn qris_mpm_get(req_body: String) -> impl Responder {
    let result = service::qris::mpm_parser::get_result(&req_body);
    HttpResponse::Ok().json(&result)
}

// pub(crate) async fn qris_mpm_post(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }
// 
// pub(crate) async fn qris_cpm() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }