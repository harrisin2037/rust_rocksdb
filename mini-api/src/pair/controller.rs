use crate::pair::dao::{RocksDBStorage, RocksDBStorageImpl};
use actix_web::{web::{Data, Path}, HttpResponse};
use json::parse;

pub async fn get(key: Path<String>, db: Data<RocksDBStorage>) -> HttpResponse {
    match &db.query(&key.into_inner()) {
        Some(v) => {
            parse(v)
                .map(
                    |
                    obj| 
                    HttpResponse::
                    Ok()
                    .content_type("application/json")
                    .body(obj.dump()))
                .unwrap_or(
                    HttpResponse::
                    InternalServerError()
                    .content_type("application/json")
                    .finish())
        }
        None    => HttpResponse::
                    NotFound()
                    .content_type("application/json")
                    .finish()
    }
}
