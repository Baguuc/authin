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
