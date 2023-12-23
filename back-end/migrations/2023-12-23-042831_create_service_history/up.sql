CREATE TABLE service_history (
	id INTEGER PRIMARY KEY,
	service_id INTEGER NOT NULL REFERENCES service(id),
	field_name TEXT NOT NULL,
	old_value TEXT,
	new_value TEXT,
	operation INTEGER REFERENCES operation(id),
	timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
