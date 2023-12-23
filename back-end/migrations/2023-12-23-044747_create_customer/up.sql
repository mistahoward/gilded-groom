CREATE TABLE customer (
	id INTEGER PRIMARY KEY,
	name TEXT NOT NULL,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL,
	email TEXT NOT NULL UNIQUE,
	phone_number TEXT NOT NULL UNIQUE,
	discovery_method_id INTEGER NOT NULL REFERENCES discovery_method(id),
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
