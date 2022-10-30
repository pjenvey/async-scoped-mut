use async_trait::async_trait;
use diesel::{MysqlConnection, r2d2::{ConnectionManager, PooledConnection}};
use lazy_static::lazy_static;
//use tokio::task;

lazy_static! {
    static ref POOL: rayon::ThreadPool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
}

type DbError = std::io::Error;
type DbResult<T> = Result<T, DbError>;
type Conn = PooledConnection<ConnectionManager<MysqlConnection>>;

pub struct Info;
pub struct Params;
pub struct PostResult;

#[async_trait]
pub trait Db {
    async fn begin(&mut self, opt: bool) -> DbResult<()>;

    async fn post(&mut self, params: Params) -> DbResult<PostResult>;

    fn info(&mut self) -> Info;
}

struct MysqlDb {
    pub _conn: Conn,
}

impl MysqlDb {
    fn begin_sync(&mut self, _opt: bool) -> DbResult<()> {
        Ok(())
    }

    fn post_sync(&mut self, _params: Params) -> DbResult<PostResult> {
        Ok(PostResult)
    }
}

#[async_trait]
impl Db for MysqlDb {
    async fn begin(&mut self, opt: bool) -> DbResult<()> {
        run_on_blocking_threadpool(move || self.begin_sync(opt)).await
    }

    async fn post(&mut self, params: Params) -> DbResult<PostResult> {
        run_on_blocking_threadpool(move || self.post_sync(params)).await
    }

    fn info(&mut self) -> Info {
        Info
    }
}

struct SpannerDb;

impl SpannerDb {
    async fn begin_async(&mut self, _opt: bool) -> DbResult<()> {
        Ok(())
    }

    async fn post_async(&mut self, _params: Params) -> DbResult<PostResult> {
        Ok(PostResult)
    }
}

#[async_trait]
impl Db for SpannerDb {
    async fn begin(&mut self, opt: bool) -> DbResult<()> {
        self.begin_async(opt).await
    }

    async fn post(&mut self, params: Params) -> DbResult<PostResult> {
        self.post_async(params).await
    }

    fn info(&mut self) -> Info {
        Info
    }
}

pub async fn run_on_blocking_threadpool<F, T>(f: F) -> Result<T, DbError>
where
    F: FnOnce() -> Result<T, DbError> + Send,
    T: Send,
{
    // XXX: not async... (needs a oneshot)
    let mut result = None;
    POOL.scope(|s| s.spawn(|_| result = Some(f())));
    result.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
