use rocksdb::DB;
use std::sync::Arc;
use log::{info, warn};
use trace::trace;
use std::fmt::Debug;

trace::init_depth_var!();

pub trait RocksDBStorageImpl {
    fn init(f_path: &str) -> Self;
    fn mutate(&self, k: &str, v: &str) -> bool;
    fn query(&self, k: &str) -> Option<String>;
    fn remove(&self, k: &str) -> bool;
}

#[derive(Clone, Debug)]
pub struct RocksDBStorage {
    db: Arc<DB>,
}

impl RocksDBStorageImpl for RocksDBStorage {

    #[trace(prefix_enter="[ENTER]", prefix_exit="[EXIT]")]
    fn init(f_path: &str) -> Self {
        RocksDBStorage { db: Arc::new(DB::open_default(f_path).unwrap()) }
    }

    #[trace(prefix_enter="[ENTER]", prefix_exit="[EXIT]")]
    fn mutate(&self, k: &str, v: &str) -> bool {
        self.db.put(k.as_bytes(), v.as_bytes()).is_ok()
    }

    #[trace(prefix_enter="[ENTER]", prefix_exit="[EXIT]")]
    fn query(&self, k: &str) -> Option<String> {
        match self.db.get(k.as_bytes()) {
            Ok(Some(v)) => {
                let result = String::from_utf8(v).unwrap();
                info!("key: {}, result: {}", k, result);
                Some(result)
            },
            Ok(None) => {
                info!("key: {}, result: None", k);
                None
            },
            Err(e) => {
                warn!("key: {}, err: {}", k, e);
                None
            }
        }
    }

    #[trace(prefix_enter="[ENTER]", prefix_exit="[EXIT]")]
    fn remove(&self, k: &str) -> bool {
        self.db.delete(k.as_bytes()).is_ok()
    }
}
