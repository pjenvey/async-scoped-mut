//use async_trait::async_trait;
use futures::future::LocalBoxFuture;

type DbResult<T> = Result<T, std::io::Error>;
type DbFuture<'a, T> = LocalBoxFuture<'a, DbResult<T>>;

pub struct Info;
pub struct Params;
pub struct PostResult;

pub trait Db<'a>: 'a {
    fn begin(&self, opt: bool) -> DbFuture<'_, ()>;

    fn post(&self, params: Params) -> DbFuture<'_, PostResult>;

    fn info(&self) -> Info;
}

struct MysqlDb;

impl MysqlDb {

    async fn begin_async(&self, _opt: bool) -> DbResult<()> {
        Ok(())
    }

    async fn post_async(&self, _params: Params) -> DbResult<PostResult> {
        // XXX: run on blocking thread pool
        Ok(PostResult)
    }

}

impl<'a> Db<'a> for MysqlDb {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
    }
}
