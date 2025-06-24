--! insert_group
INSERT INTO groups (name) VALUES (:name);

--! retrieve_group
SELECT 
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
;


--! list_groups
SELECT 
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
;

--! delete_group
DELETE FROM groups WHERE name = :name;

--! grant_group
INSERT INTO user_groups (user_login, group_name) VALUES (:login, :group_name);

--! revoke_group
DELETE FROM user_groups WHERE user_login = :login AND group_name = :group_name;
