--! insert_user
INSERT INTO users (login, pwd) VALUES (:login, :pwd);

--! retrieve_user
SELECT id, login, pwd FROM users WHERE login = :login;

--! delete_user
DELETE FROM users WHERE login = :login;
