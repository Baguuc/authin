--! insert_user
INSERT INTO users (login, pwd) VALUES (:login, :pwd);

--! insert_user_no_pwd
INSERT INTO users (login, pwd) VALUES (:login, '');

--! retrieve_user
SELECT id, login, pwd FROM users WHERE login = :login;

--! list_users
SELECT 
  u.login,
  ARRAY_REMOVE(ARRAY_AGG(ug.group_name), NULL) AS groups
FROM 
  users u
LEFT JOIN 
  user_groups ug 
  ON 
  ug.user_login = u.login
GROUP BY
  u.login
;

--! delete_user
DELETE FROM users WHERE login = :login;

--! retrieve_user_permission
SELECT 
  p.name
FROM 
  permissions p 
INNER JOIN 
  group_permissions gp 
  ON 
  p.name = gp.permission_name
INNER JOIN 
  groups g
  ON 
  g.name = gp.group_name
INNER JOIN 
  user_groups ug
  ON
  g.name = ug.group_name
INNER JOIN
  users u
  ON
  u.login = ug.user_login
WHERE 
  u.login = :login 
  AND
  p.name = :permission_name;

