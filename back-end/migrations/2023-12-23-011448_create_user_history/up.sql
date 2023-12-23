CREATE TABLE operation (
	id INTEGER PRIMARY KEY,
	name TEXT NOT NULL,
	description TEXT
);

INSERT INTO operation (name, description) VALUES ('CREATE', 'Create a new record');
INSERT INTO operation (name, description) VALUES ('UPDATE', 'Update an existing record');
INSERT INTO operation (name, description) VALUES ('DELETE', 'Delete an existing record');

CREATE TABLE user_history (
	id INTEGER PRIMARY KEY,
	user_id INTEGER NOT NULL,
	field_name TEXT NOT NULL,
	old_value TEXT,
	new_value TEXT,
	operation INTEGER REFERENCES operation(id) NOT NULL,
	timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
