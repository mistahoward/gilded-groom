CREATE TABLE user (
	id INTEGER PRIMARY KEY,
	name TEXT NOT NULL,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL,
	email TEXT NOT NULL UNIQUE,
	password TEXT NOT NULL,
	salt TEXT NOT NULL,
	created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
	last_login INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);
