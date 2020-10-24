extern crate reqwest;
use crate::pair::dto::{Pair, PairImpl};
use crate::pair::dao::{RocksDBStorage, RocksDBStorageImpl};
use anyhow::{anyhow, Result, Error};
use background_jobs::memory_storage::Storage;
use futures::future::{ok, err, Ready};
use background_jobs::{
    create_server, 
    Job, 
    MaxRetries, 
    WorkerConfig,
    Backoff,
};

const PAIR_QUEUE: &str = "pair_queue";

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CrawlPairDataJob {
    url: String,
}

impl CrawlPairDataJob {
    pub fn new(url: String) -> Self {
        CrawlPairDataJob {
            url,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CrawlPairDataJobState {
    rocksdb_file_path: String,
}

pub async fn exec(rocksdb_file_path: String, url: String) -> Result<(), Error> {

    let storage = Storage::new();
    let queue_handle = create_server(storage);

    WorkerConfig::new(move || CrawlPairDataJobState::
        new(rocksdb_file_path.to_string()))
        .register::<CrawlPairDataJob>()
        .set_worker_count(PAIR_QUEUE, 16)
        .start(queue_handle.clone());

    queue_handle.queue(CrawlPairDataJob::new(url))?;

    actix_rt::signal::ctrl_c().await?;
    Ok(())
}

impl CrawlPairDataJobState {
    pub fn new(rocksdb_file_path: String) -> Self {
        CrawlPairDataJobState {
            rocksdb_file_path,
        }
    }
}

impl Job for CrawlPairDataJob {

    type State = CrawlPairDataJobState;
    type Future = Ready<Result<(), Error>>;

    const NAME: &'static str = "crawl_pair_data_job";
    const QUEUE: &'static str = PAIR_QUEUE;
    const MAX_RETRIES: MaxRetries = MaxRetries::Count(1);
    const BACKOFF: Backoff = Backoff::Exponential(5);
    const TIMEOUT: i64 = 15_000;

    fn queue(&self) -> &str {
        Self::QUEUE
    }

    fn max_retries(&self) -> MaxRetries {
        Self::MAX_RETRIES
    }

    fn backoff_strategy(&self) -> Backoff {
        Self::BACKOFF
    }

    fn timeout(&self) -> i64 {
        Self::TIMEOUT
    }

    fn run(self, state: Self::State) -> Self::Future {
        let pair = fetchPairData(self.url).unwrap();
        if pair.lprice == "" {
            return err(anyhow!("cannot get response"));
        }
        let db: RocksDBStorage = RocksDBStorageImpl::init(&state.rocksdb_file_path);

        let b: bool = storePairToRocksDB(db, pair);
        match b {
            true => {ok(())}
            false => {err(anyhow!("cannot access db, with returning value: ({:?})", b))}
        }
    }
}

fn fetchPairData(url: String) -> Result<Pair, Box<dyn std::error::Error>> {
    let resp: Pair = reqwest::blocking::get(&url)?.json()?;
    Ok(resp)
}

fn storePairToRocksDB(db: RocksDBStorage, pair: Pair) -> bool {

    let mut key: String = pair.curr1.to_owned();
    key.push_str(":");
    key.push_str(&pair.curr2);
    let value: String = pair.lprice;

    return db.mutate(&key, &value);
}

