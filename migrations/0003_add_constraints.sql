ALTER TABLE group_permissions ADD CONSTRAINT pair_unique UNIQUE (group_name, permission_name);
ALTER TABLE user_groups ADD CONSTRAINT pair_unique UNIQUE (user_login, group_name);
