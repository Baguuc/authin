// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct GrantGroupParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub login: T1,
    pub group_name: T2,
}
#[derive(Debug)]
pub struct RevokeGroupParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub login: T1,
    pub group_name: T2,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct StringQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<&str, tokio_postgres::Error>,
    mapper: fn(&str) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> StringQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'c, 'a, 's, C, R, N> {
        StringQuery {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(it)
    }
}
pub fn insert_group() -> InsertGroupStmt {
    InsertGroupStmt(crate::client::async_::Stmt::new(
        "INSERT INTO groups (name) VALUES ($1)",
    ))
}
pub struct InsertGroupStmt(crate::client::async_::Stmt);
impl InsertGroupStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        name: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[name]).await
    }
}
pub fn retrieve_group() -> RetrieveGroupStmt {
    RetrieveGroupStmt(crate::client::async_::Stmt::new(
        "SELECT name FROM groups WHERE name = $1",
    ))
}
pub struct RetrieveGroupStmt(crate::client::async_::Stmt);
impl RetrieveGroupStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        name: &'a T1,
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [name],
            stmt: &mut self.0,
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub fn list_groups() -> ListGroupsStmt {
    ListGroupsStmt(crate::client::async_::Stmt::new("SELECT name FROM groups"))
}
pub struct ListGroupsStmt(crate::client::async_::Stmt);
impl ListGroupsStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> StringQuery<'c, 'a, 's, C, String, 0> {
        StringQuery {
            client,
            params: [],
            stmt: &mut self.0,
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub fn delete_group() -> DeleteGroupStmt {
    DeleteGroupStmt(crate::client::async_::Stmt::new(
        "DELETE FROM groups WHERE name = $1",
    ))
}
pub struct DeleteGroupStmt(crate::client::async_::Stmt);
impl DeleteGroupStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        name: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[name]).await
    }
}
pub fn grant_group() -> GrantGroupStmt {
    GrantGroupStmt(crate::client::async_::Stmt::new(
        "INSERT INTO user_groups (user_login, group_name) VALUES ($1, $2)",
    ))
}
pub struct GrantGroupStmt(crate::client::async_::Stmt);
impl GrantGroupStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        login: &'a T1,
        group_name: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[login, group_name]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        GrantGroupParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for GrantGroupStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GrantGroupParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.login, &params.group_name))
    }
}
pub fn revoke_group() -> RevokeGroupStmt {
    RevokeGroupStmt(crate::client::async_::Stmt::new(
        "DELETE FROM user_groups WHERE user_login = $1 AND group_name = $2",
    ))
}
pub struct RevokeGroupStmt(crate::client::async_::Stmt);
impl RevokeGroupStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        login: &'a T1,
        group_name: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[login, group_name]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        RevokeGroupParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for RevokeGroupStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a RevokeGroupParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.login, &params.group_name))
    }
}
