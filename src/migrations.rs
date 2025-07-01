use crate::prelude::*;

const MIGRATIONS: [&str; 3] = [
"    
CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  login VARCHAR(32) NOT NULL UNIQUE,
  pwd VARCHAR(97) NOT NULL
);

CREATE INDEX IF NOT EXISTS users_login_index ON users USING HASH (login); 

CREATE TYPE TUSER AS (
  id INTEGER,
  login VARCHAR(32),
  pwd VARCHAR(80)
);
",

"
CREATE TABLE IF NOT EXISTS groups (
  name TEXT PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS user_groups (
  user_login TEXT NOT NULL,
  group_name TEXT NOT NULL,

  FOREIGN KEY (user_login) REFERENCES users(login) ON DELETE CASCADE,
  FOREIGN KEY (group_name) REFERENCES groups(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS permissions (
  name TEXT PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS group_permissions (
  group_name TEXT NOT NULL,
  permission_name TEXT NOT NULL,

  FOREIGN KEY (group_name) REFERENCES groups(name) ON DELETE CASCADE,
  FOREIGN KEY (permission_name) REFERENCES permissions(name) ON DELETE CASCADE
);
",

"
ALTER TABLE group_permissions ADD CONSTRAINT pair_unique UNIQUE (group_name, permission_name);
ALTER TABLE user_groups ADD CONSTRAINT pair_unique UNIQUE (user_login, group_name);
"
];

pub fn migrate() -> Result<()> { 
    for migration in MIGRATIONS {
        println!("{}", migration);
    } 
    
    return Ok(());
}
