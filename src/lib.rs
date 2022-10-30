use async_trait::async_trait;
use tokio::task;

type DbError = std::io::Error;
type DbResult<T> = Result<T, DbError>;

pub struct Info;
pub struct Params;
pub struct PostResult;

#[async_trait]
pub trait Db {
    async fn begin(&mut self, opt: bool) -> DbResult<()>;

    async fn post(&mut self, params: Params) -> DbResult<PostResult>;

    fn info(&mut self) -> Info;
}

#[derive(Clone)]
struct MysqlDb;

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
        let mut db = self.clone();
        run_on_blocking_threadpool(move || db.begin_sync(opt)).await
    }

    async fn post(&mut self, params: Params) -> DbResult<PostResult> {
        let mut db = self.clone();
        run_on_blocking_threadpool(move || db.post_sync(params)).await
    }

    fn info(&mut self) -> Info {
        Info
    }
}

#[derive(Clone)]
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
