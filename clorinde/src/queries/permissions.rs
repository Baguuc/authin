// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct GrantPermissionParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub group_name: T1,
    pub permission_name: T2,
}
#[derive(Debug)]
pub struct RevokePermissionParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub group_name: T1,
    pub permission_name: T2,
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
pub fn insert_permission() -> InsertPermissionStmt {
    InsertPermissionStmt(crate::client::async_::Stmt::new(
        "INSERT INTO permissions (name) VALUES ($1)",
    ))
}
pub struct InsertPermissionStmt(crate::client::async_::Stmt);
impl InsertPermissionStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        name: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[name]).await
    }
}
pub fn retrieve_permission() -> RetrievePermissionStmt {
    RetrievePermissionStmt(crate::client::async_::Stmt::new(
        "SELECT name FROM permissions WHERE name = $1",
    ))
}
pub struct RetrievePermissionStmt(crate::client::async_::Stmt);
impl RetrievePermissionStmt {
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
pub fn delete_permission() -> DeletePermissionStmt {
    DeletePermissionStmt(crate::client::async_::Stmt::new(
        "DELETE FROM permissions WHERE name = $1",
    ))
}
pub struct DeletePermissionStmt(crate::client::async_::Stmt);
impl DeletePermissionStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        name: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[name]).await
    }
}
pub fn grant_permission() -> GrantPermissionStmt {
    GrantPermissionStmt(crate::client::async_::Stmt::new(
        "INSERT INTO group_permissions (group_name, permission_name) VALUES ($1, $2)",
    ))
}
pub struct GrantPermissionStmt(crate::client::async_::Stmt);
impl GrantPermissionStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        group_name: &'a T1,
        permission_name: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[group_name, permission_name]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        GrantPermissionParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for GrantPermissionStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a GrantPermissionParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.group_name, &params.permission_name))
    }
}
pub fn revoke_permission() -> RevokePermissionStmt {
    RevokePermissionStmt(crate::client::async_::Stmt::new(
        "DELETE FROM group_permissions WHERE group_name = $1 AND permission_name = $2",
    ))
}
pub struct RevokePermissionStmt(crate::client::async_::Stmt);
impl RevokePermissionStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        group_name: &'a T1,
        permission_name: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[group_name, permission_name]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        RevokePermissionParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for RevokePermissionStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a RevokePermissionParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.group_name, &params.permission_name))
    }
}
