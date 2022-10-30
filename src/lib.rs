//use async_trait::async_trait;
use futures::future::LocalBoxFuture;
use tokio::task;

type DbError = std::io::Error;
type DbResult<T> = Result<T, DbError>;
type DbFuture<'a, T> = LocalBoxFuture<'a, DbResult<T>>;

pub struct Info;
pub struct Params;
pub struct PostResult;

pub trait Db<'a>: 'a {
    fn begin(&self, opt: bool) -> DbFuture<'_, ()>;

    fn post(&self, params: Params) -> DbFuture<'_, PostResult>;

    fn info(&self) -> Info;
}

#[derive(Clone)]
struct MysqlDb;

impl MysqlDb {
    fn begin_sync(&self, _opt: bool) -> DbResult<()> {
        Ok(())
    }

    fn post_sync(&self, _params: Params) -> DbResult<PostResult> {
        Ok(PostResult)
    }
}

impl<'a> Db<'a> for MysqlDb {
    fn begin(&self, opt: bool) -> DbFuture<'_, ()> {
        let db = self.clone();
        Box::pin(run_on_blocking_threadpool(move || db.begin_sync(opt)))
    }

    fn post(&self, params: Params) -> DbFuture<'_, PostResult> {
        let db = self.clone();
        Box::pin(run_on_blocking_threadpool(move || db.post_sync(params)))
    }

    fn info(&self) -> Info {
        Info
    }
}

#[derive(Clone)]
struct SpannerDb;

impl SpannerDb {
    async fn begin_async(&self, _opt: bool) -> DbResult<()> {
        Ok(())
    }

    async fn post_async(&self, _params: Params) -> DbResult<PostResult> {
        Ok(PostResult)
    }
}

impl<'a> Db<'a> for SpannerDb {
    fn begin(&self, opt: bool) -> DbFuture<'_, ()> {
        let db = self.clone();
        Box::pin(async move { db.begin_async(opt).await })
    }

    fn post(&self, params: Params) -> DbFuture<'_, PostResult> {
        let db = self.clone();
        Box::pin(async move { db.post_async(params).await })
    }

    fn info(&self) -> Info {
        Info
    }
}

pub async fn run_on_blocking_threadpool<F, T>(f: F) -> Result<T, DbError>
where
    F: FnOnce() -> Result<T, DbError> + Send + 'static,
    T: Send + 'static,
{
    task::spawn_blocking(f).await?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
