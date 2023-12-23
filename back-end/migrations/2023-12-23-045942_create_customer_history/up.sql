CREATE TABLE customer_history (
	id INTEGER PRIMARY KEY,
	customer_id INTEGER NOT NULL REFERENCES customer(id),
	field_name TEXT NOT NULL,
	old_value TEXT,
	new_value TEXT,
	operation INTEGER REFERENCES operation(id),
	timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
