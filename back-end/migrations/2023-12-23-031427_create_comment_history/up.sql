CREATE TABLE comment_history (
	id INTEGER PRIMARY KEY,
	comment_id INTEGER NOT NULL REFERENCES comment(id),
	field_name TEXT NOT NULL,
	old_value TEXT,
	new_value TEXT,
	operation INTEGER REFERENCES operation(id),
	timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
