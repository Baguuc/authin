use crate::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, sqlx::FromRow)]
pub struct Permission {
    pub name: String,
}

impl Permission {
    pub async fn insert<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "INSERT INTO permissions (name) VALUES ($1);";
        let result = query(sql).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn retrieve<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<Self> {
        use sqlx::query_as;

        let sql = "SELECT * FROM permissions WHERE name = $1;";
        let result = query_as(sql).bind(name).fetch_one(client).await;

        match result {
            Ok(permission) => return Ok(permission),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn list<'c, C: sqlx::postgres::PgExecutor<'c>>(client: C) -> Result<Vec<Self>> {
        use sqlx::query_as;

        let sql = "SELECT * FROM permissions;";
        let result = query_as(sql).fetch_all(client).await;

        match result {
            Ok(permissions) => return Ok(permissions),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn delete<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "DELETE FROM permissions WHERE name = $1;";
        let result = query(sql).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn grant<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, group_name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "INSERT INTO group_permissions (group_name, permission_name) VALUES ($1, $2);";
        let result = query(sql).bind(group_name).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn revoke<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, group_name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "DELETE FROM group_permissions WHERE group_name = $1 AND permission_name = $2;";
        let result = query(sql).bind(group_name).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }

    pub async fn sync(new_permissions: &Vec<String>, client: &sqlx::postgres::PgPool) -> Result<()> {
        use sqlx::query;

        let mut tx = client.begin().await?;
        
        query("SET CONSTRAINTS ALL DEFERRED;").execute(&mut *tx).await;

        let current_permissions = Self::list(&mut *tx)
            .await?
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>();

        for permission in &current_permissions {
            if !new_permissions.contains(permission) {
                let _ = Self::delete(permission, &mut *tx)
                    .await?;
            }
        }

        for permission in new_permissions {
            if !current_permissions.contains(&permission) {
                let _ = Self::insert(permission, &mut *tx)
                    .await?;
            }
        }

        let _ = tx.commit().await?;
        
        return Ok(());
    }
}
