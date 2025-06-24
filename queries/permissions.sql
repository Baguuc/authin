--! insert_permission
INSERT INTO permissions (name) VALUES (:name);

--! retrieve_permission
SELECT name FROM permissions WHERE name = :name;

--! delete_permission
DELETE FROM permissions WHERE name = :name;

--! grant_permission
INSERT INTO group_permissions (group_name, permission_name) VALUES (:group_name, :permission_name);

--! revoke_permission
DELETE FROM group_permissions WHERE group_name = :group_name AND permission_name = :permission_name;
