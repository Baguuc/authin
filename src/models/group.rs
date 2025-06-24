use crate::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Group {
    pub name: String,
    pub permissions: Vec<String>
}

pub async fn sync_groups(client: &clorinde::deadpool_postgres::Client, new_groups: &Vec<Group>) -> Result<()> {
    use clorinde::queries::{permissions::{grant_permission, revoke_permission}, groups::{list_groups, insert_group, delete_group}};

    let current_groups = list_groups()
        .bind(client)
        .all()
        .await?
        .iter()
        .map(|g| Group { 
            name: g.name.clone(),
            permissions: g.permissions.clone()
        })
        .collect::<Vec<Group>>();

    for group in &current_groups {
        let mut found = false;
        
        for n_group in new_groups {
            if group.name == n_group.name { 
                found = true;
                break;
            }
        }

        if !found {
            let current_groups = delete_group()
                .bind(client, &group.name)
                .await?;
        }
    }

    for group in new_groups {
        println!("{:?}", group);
        let mut found = false;
        
        for c_group in &current_groups {
            if group.name == c_group.name {
                // only permissions do not match
                found = true;

                if group.permissions == c_group.permissions {
                    break;
                }

                for permission in &c_group.permissions {                     
                    revoke_permission()
                        .bind(client, &c_group.name, &permission)
                        .await;
                }
                
                for permission in &group.permissions {                     
                    grant_permission()
                        .bind(client, &group.name, &permission)
                        .await;
                }
                
                break;
            } 
        }

        if found { continue; }
        
        // if couldn't be found just add it from scratch
        insert_group()
            .bind(client, &group.name)
            .await?;

        for permission in &group.permissions {
            grant_permission()
                .bind(client, &group.name, &permission)
                .await?;
        }
    }
    
    return Ok(());
}
