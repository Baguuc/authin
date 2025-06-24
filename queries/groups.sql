--! insert_group
INSERT INTO groups (name) VALUES (:name);

--! retrieve_group
SELECT name FROM groups WHERE name = :name;

--! list_groups
SELECT name FROM groups;

--! delete_group
DELETE FROM groups WHERE name = :name;

--! grant_group
INSERT INTO user_groups (user_login, group_name) VALUES (:login, :group_name);

--! revoke_group
DELETE FROM user_groups WHERE user_login = :login AND group_name = :group_name;
