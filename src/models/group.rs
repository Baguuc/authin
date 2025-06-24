use crate::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Group {
    pub name: String,
    pub permissions: Vec<String>
}

pub async fn sync_groups(client: &clorinde::deadpool_postgres::Client, new_groups: &Vec<Group>) -> Result<()> {
    use clorinde::queries::{permissions::grant_permission, groups::{list_groups, insert_group, delete_group}};

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
        if !new_groups.contains(group) {
            let current_groups = delete_group()
                .bind(client, &group.name)
                .await?;
        }
    }

    for group in new_groups {
        if !current_groups.contains(&group) {
            let current_groups = insert_group()
                .bind(client, &group.name)
                .await?;

            for permission in &group.permissions {
                grant_permission()
                    .bind(client, &group.name, &permission)
                    .await?;
            }
        }
    }
    
    return Ok(());
}
