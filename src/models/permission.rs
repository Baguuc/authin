use crate::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Permission {
    pub name: String,
}

pub async fn sync_permissions<C: clorinde::client::GenericClient>(client: &C, new_permissions: &Vec<String>) -> Result<()> {
    use clorinde::queries::permissions::{list_permissions,insert_permission,delete_permission};

    let current_permissions = list_permissions()
        .bind(client)
        .all()
        .await?;

    for permission in &current_permissions {
        if !new_permissions.contains(permission) {
            let current_permissions = delete_permission()
                .bind(client, &permission)
                .await?;
        }
    }

    for permission in new_permissions {
        if !current_permissions.contains(&permission) {
            let current_permissions = insert_permission()
                .bind(client, &permission)
                .await?;
        }
    }
    
    return Ok(());
}
