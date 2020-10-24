#[macro_use] extern crate lazy_static;
extern crate env_logger;
extern crate failure;
extern crate envconfig;
extern crate envconfig_derive;
extern crate rocksdb;
extern crate serde_derive;
use actix_web::{middleware, web, HttpServer, App};
use configs::Conf;
use envconfig::Envconfig;
mod configs;
mod pair;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    let config = Conf::init_from_env().unwrap();
    let json_limit: usize = config.json_limit_size.parse().unwrap();

    let db: pair::dao::RocksDBStorage = pair::dao::RocksDBStorageImpl::init(&config.rocksdb_file_path);

    let result = pair::background::exec(config.rocksdb_file_path, config.pair_url);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(json_limit))
            .data(db.clone())
            .service(web::scope("/v1")
                    .service(
                        web::resource("/pair")
                            .route(web::get().to(pair::controller::get)),
                    ),
            )
    })
    .bind(&config.bind_address)?
    .run()
    .await

}
