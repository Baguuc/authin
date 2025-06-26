// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct InsertUserParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub login: T1,
    pub pwd: T2,
}
#[derive(Debug)]
pub struct UpdateUserPwdParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub pwd: T1,
    pub login: T2,
}
#[derive(Debug)]
pub struct RetrieveUserPermissionParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub login: T1,
    pub permission_name: T2,
}
#[derive(Debug, Clone, PartialEq)]
pub struct RetrieveUser {
    pub id: i32,
    pub login: String,
    pub pwd: String,
}
pub struct RetrieveUserBorrowed<'a> {
    pub id: i32,
    pub login: &'a str,
    pub pwd: &'a str,
}
impl<'a> From<RetrieveUserBorrowed<'a>> for RetrieveUser {
    fn from(RetrieveUserBorrowed { id, login, pwd }: RetrieveUserBorrowed<'a>) -> Self {
        Self {
            id,
            login: login.into(),
            pwd: pwd.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ListUsers {
    pub login: String,
    pub groups: Vec<String>,
}
pub struct ListUsersBorrowed<'a> {
    pub login: &'a str,
    pub groups: crate::ArrayIterator<'a, &'a str>,
}
impl<'a> From<ListUsersBorrowed<'a>> for ListUsers {
    fn from(ListUsersBorrowed { login, groups }: ListUsersBorrowed<'a>) -> Self {
        Self {
            login: login.into(),
            groups: groups.map(|v| v.into()).collect(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct RetrieveUserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<RetrieveUserBorrowed, tokio_postgres::Error>,
    mapper: fn(RetrieveUserBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> RetrieveUserQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(RetrieveUserBorrowed) -> R,
    ) -> RetrieveUserQuery<'c, 'a, 's, C, R, N> {
        RetrieveUserQuery {
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
pub struct ListUsersQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<ListUsersBorrowed, tokio_postgres::Error>,
    mapper: fn(ListUsersBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ListUsersQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(ListUsersBorrowed) -> R) -> ListUsersQuery<'c, 'a, 's, C, R, N> {
        ListUsersQuery {
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
pub fn insert_user() -> InsertUserStmt {
    InsertUserStmt(crate::client::async_::Stmt::new(
        "INSERT INTO users (login, pwd) VALUES ($1, $2)",
    ))
}
pub struct InsertUserStmt(crate::client::async_::Stmt);
impl InsertUserStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        login: &'a T1,
        pwd: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[login, pwd]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        InsertUserParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for InsertUserStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertUserParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.login, &params.pwd))
    }
}
pub fn insert_user_no_pwd() -> InsertUserNoPwdStmt {
    InsertUserNoPwdStmt(crate::client::async_::Stmt::new(
        "INSERT INTO users (login, pwd) VALUES ($1, '')",
    ))
}
pub struct InsertUserNoPwdStmt(crate::client::async_::Stmt);
impl InsertUserNoPwdStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        login: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[login]).await
    }
}
pub fn retrieve_user() -> RetrieveUserStmt {
    RetrieveUserStmt(crate::client::async_::Stmt::new(
        "SELECT id, login, pwd FROM users WHERE login = $1",
    ))
}
pub struct RetrieveUserStmt(crate::client::async_::Stmt);
impl RetrieveUserStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        login: &'a T1,
    ) -> RetrieveUserQuery<'c, 'a, 's, C, RetrieveUser, 1> {
        RetrieveUserQuery {
            client,
            params: [login],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<RetrieveUserBorrowed, tokio_postgres::Error> {
                    Ok(RetrieveUserBorrowed {
                        id: row.try_get(0)?,
                        login: row.try_get(1)?,
                        pwd: row.try_get(2)?,
                    })
                },
            mapper: |it| RetrieveUser::from(it),
        }
    }
}
pub fn list_users() -> ListUsersStmt {
    ListUsersStmt(crate::client::async_::Stmt::new(
        "SELECT u.login, ARRAY_REMOVE(ARRAY_AGG(ug.group_name), NULL) AS groups FROM users u LEFT JOIN user_groups ug ON ug.user_login = u.login GROUP BY u.login",
    ))
}
pub struct ListUsersStmt(crate::client::async_::Stmt);
impl ListUsersStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> ListUsersQuery<'c, 'a, 's, C, ListUsers, 0> {
        ListUsersQuery {
            client,
            params: [],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<ListUsersBorrowed, tokio_postgres::Error> {
                    Ok(ListUsersBorrowed {
                        login: row.try_get(0)?,
                        groups: row.try_get(1)?,
                    })
                },
            mapper: |it| ListUsers::from(it),
        }
    }
}
pub fn update_user_pwd() -> UpdateUserPwdStmt {
    UpdateUserPwdStmt(crate::client::async_::Stmt::new(
        "UPDATE users SET pwd = $1 WHERE login = $2",
    ))
}
pub struct UpdateUserPwdStmt(crate::client::async_::Stmt);
impl UpdateUserPwdStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        pwd: &'a T1,
        login: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[pwd, login]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateUserPwdParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateUserPwdStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a UpdateUserPwdParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.pwd, &params.login))
    }
}
pub fn delete_user() -> DeleteUserStmt {
    DeleteUserStmt(crate::client::async_::Stmt::new(
        "DELETE FROM users WHERE login = $1",
    ))
}
pub struct DeleteUserStmt(crate::client::async_::Stmt);
impl DeleteUserStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        login: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[login]).await
    }
}
pub fn retrieve_user_permission() -> RetrieveUserPermissionStmt {
    RetrieveUserPermissionStmt(crate::client::async_::Stmt::new(
        "SELECT p.name FROM permissions p INNER JOIN group_permissions gp ON p.name = gp.permission_name INNER JOIN groups g ON g.name = gp.group_name INNER JOIN user_groups ug ON g.name = ug.group_name INNER JOIN users u ON u.login = ug.user_login WHERE u.login = $1 AND p.name = $2",
    ))
}
pub struct RetrieveUserPermissionStmt(crate::client::async_::Stmt);
impl RetrieveUserPermissionStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        login: &'a T1,
        permission_name: &'a T2,
    ) -> StringQuery<'c, 'a, 's, C, String, 2> {
        StringQuery {
            client,
            params: [login, permission_name],
            stmt: &mut self.0,
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        RetrieveUserPermissionParams<T1, T2>,
        StringQuery<'c, 'a, 's, C, String, 2>,
        C,
    > for RetrieveUserPermissionStmt
{
    fn params(
        &'s mut self,
        client: &'c C,
        params: &'a RetrieveUserPermissionParams<T1, T2>,
    ) -> StringQuery<'c, 'a, 's, C, String, 2> {
        self.bind(client, &params.login, &params.permission_name)
    }
}
