use actix_web::{HttpResponse, web::Path};

pub async fn hi(path: Path<String>) -> HttpResponse {
  HttpResponse::Ok().body(format!("Hi, {}", path))
}