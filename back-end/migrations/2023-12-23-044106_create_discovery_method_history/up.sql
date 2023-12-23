CREATE TABLE discovery_method_history (
	id INTEGER PRIMARY KEY,
	discovery_method_id INTEGER NOT NULL REFERENCES discovery_method(id),
	field_name TEXT NOT NULL,
	old_value TEXT,
	new_value TEXT,
	operation INTEGER REFERENCES operation(id),
	timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
