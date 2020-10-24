extern crate envconfig_derive;
extern crate envconfig;

use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Conf {

    #[envconfig(from = "BIND_ADDRESS", default = "127.0.0.1:3000")]
    pub bind_address: String,
    
    #[envconfig(from = "JSON_LIMIT_SIZE")]
    pub json_limit_size: String,

    #[envconfig(from = "ROCKSDB_FILE_PATH", default = "/tmp/rocks/db")]
    pub rocksdb_file_path: String,

    #[envconfig(from = "PAIR_URL")]
    pub pair_url: String,
}
