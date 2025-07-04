use crate::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug, sqlx::FromRow)]
pub struct Group {
    pub name: String,
    pub permissions: Vec<String>
}

impl Group {
    pub async fn insert<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "INSERT INTO groups (name) VALUES ($1);";
        let result = query(sql).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn retrieve<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<Self> {
        use sqlx::query_as;

        let sql = "SELECT 
          g.name,
          ARRAY_REMOVE(ARRAY_AGG(gp.permission_name), NULL) AS permissions
        FROM 
          groups g
        LEFT JOIN
          group_permissions gp
          ON
          gp.group_name = g.name
        WHERE
          name = :group_name
        GROUP BY
          g.name
        ;";
        let result = query_as(sql).bind(name).fetch_one(client).await;

        match result {
            Ok(group) => return Ok(group),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn list<'c, C: sqlx::postgres::PgExecutor<'c>>(client: C) -> Result<Vec<Self>> {
        use sqlx::query_as;

        let sql = "SELECT 
          g.name,
          ARRAY_REMOVE(ARRAY_AGG(gp.permission_name), NULL) AS permissions
        FROM 
          groups g
        LEFT JOIN 
          group_permissions gp 
          ON 
          gp.group_name = g.name
        GROUP BY
          g.name
        ;";
        let result = query_as(sql).fetch_all(client).await;

        match result {
            Ok(groups) => return Ok(groups),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn delete<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "DELETE FROM groups WHERE name = $1;";
        let result = query(sql).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn grant<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, user_login: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "INSERT INTO user_groups (user_login, group_name) VALUES ($1, $2);";
        let result = query(sql).bind(user_login).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }
    
    pub async fn revoke<'c, C: sqlx::postgres::PgExecutor<'c>>(name: &String, user_login: &String, client: C) -> Result<()> {
        use sqlx::query;

        let sql = "DELETE FROM user_groups WHERE user_login = $1 AND group_name = $2;";
        let result = query(sql).bind(user_login).bind(name).execute(client).await;

        match result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(Error::Sqlx(err))
        };
    }

    pub async fn sync(new_groups: &Vec<Group>, client: &sqlx::postgres::PgPool) -> Result<()> {
        use sqlx::query;
        use crate::models::Permission;

        let mut tx = client.begin().await?;
        
        query("SET CONSTRAINTS ALL DEFERRED;").execute(&mut *tx).await;
        
        let current_groups = Self::list(&mut *tx)
            .await?;

        for group in &current_groups {
            let mut found = false;
            
            for n_group in new_groups {
                if group.name == n_group.name { 
                    found = true;
                    break;
                }
            }

            if !found {
                let current_groups = Self::delete(&group.name, &mut *tx)
                    .await?;
            }
        }

        for group in new_groups {
            let mut found = false;
            
            for c_group in &current_groups {
                if group.name == c_group.name {
                    // only permissions do not match
                    found = true;

                    if group.permissions == c_group.permissions {
                        break;
                    }

                    for permission in &c_group.permissions {                     
                        Permission::revoke(permission, &c_group.name, &mut *tx)
                            .await?;
                    }
                    
                    for permission in &group.permissions {
                        Permission::grant(permission, &group.name, &mut *tx)
                            .await?;
                    }
                    
                    break;
                } 
            }

            if found { continue; }
            
            // if couldn't be found just add it from scratch
            Self::insert(&group.name, &mut *tx)
                .await?;

            for permission in &group.permissions {
                Permission::grant(permission, &group.name, &mut *tx)
                    .await?;
            }
        }
        
        let _ = tx.commit().await?;
        
        return Ok(());
    }
}
